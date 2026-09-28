use std::str::Chars;

use super::{Token, TokenKind, Span};

pub struct ShellCursor<'src> {
   src: &'src str,
   chars: Chars<'src>,
   len: u32,
   pos: usize,
}

impl<'src> Iterator for ShellCursor<'src> {
   type Item = Token;

   fn next(&mut self) -> Option<Self::Item> {
      if let token = self.token()
         && token.kind != TokenKind::Eof
      {
         Some(token)
      } else {
         None
      }
   }
}

macro_rules! delimiters_arm {
    () => {
        '$' | '"' | '\'' | ' ' | '\t' | '\n' | '(' | ')' | '>' | '<' | '|' | '&' | ':' | ';'
    };
}

impl<'src> ShellCursor<'src> {

   pub fn new(src: &'src str) -> Self {
      let chars = src.chars();
      Self {
         src,
         chars,
         len: 0, 
         pos: 0,

      }
   }

   pub fn first(&self) -> Option<char> {
      self.chars.clone().next()
   }

   pub fn second(&self) -> Option<char> {
      let mut chars = self.chars.clone();
      chars.next();
      chars.next()
   }

   pub fn third(&self) -> Option<char> {
      let mut chars = self.chars.clone();
      chars.next();
      chars.next();
      chars.next()
   }

   pub fn bump(&mut self) {
      let char = self.chars.next();
      self.len += char.map(|char| char.len_utf8()).unwrap_or(0) as u32;
      // println!("bumped {:?}", char);
   }

   pub fn span(&mut self) -> Span {
      let len = std::mem::replace(&mut self.len, 0);
      let pos = self.pos;
      self.pos += len as usize;
      Span::new(pos, len)
   }

   fn single_byte_token(&mut self, kind: TokenKind) -> Token {
      let span = Span::one(self.pos);
      let token = Token {
         kind,
         span
      };

      self.pos += 1;
      self.chars.next();
      token
   }

   fn str(&self) -> &str {
      &self.src[self.pos..self.pos + self.len as usize]
   }

   fn shell_word(&mut self, char: char) -> Token {
      assert!(self.len == 0);

      let mut char = char;
      loop {
         match char {
            '\\' => match self.second() {
               None => break,
               Some(_) => { 
                  self.bump(); 
                  self.bump();
               }
            },
            delimiters_arm!() => break,
            _ => self.bump(),
         };

         if let Some(next) = self.first() {
            char = next;
         } else {
            break;
         };
      }

      let token= if self.str() == "fd" {
         let kind = TokenKind::Fd;
         let span = self.span();
         Token::new(kind, span)
      } else {
         let kind = TokenKind::Word;
         let span = self.span();
         Token::new(kind, span)
      };
      token
   }

   fn pipe(&mut self) -> Token {
      if let Some('|') = self.second() {
         self.bump();
         self.bump();
         let kind = TokenKind::Or;
         let span = self.span();
         Token { kind, span }
      } else {
         self.bump();
         let kind = TokenKind::Pipe;
         let span = self.span();
         Token { kind, span }
      }
   }

   fn ampersand(&mut self) -> Token {
      if let Some('&') = self.second() {
         self.bump();
         self.bump();
         let kind = TokenKind::And;
         let span = self.span();
         Token { kind, span }
      } else {
         self.bump();
         let kind = TokenKind::Ampersand;
         let span = self.span();
         Token { kind, span }
      }
   }

   fn redirect(&mut self) -> Token {
      self.bump();

      while let Some(char) = self.first() {
         match char {
            '>' | '<' => self.bump(),
            _ => break,
         };
      }

      let str = self.str();

      let kind = match str {
         ">" => TokenKind::RedirectOut,
         ">>" => TokenKind::RedirectOutAppend,
         "<" => TokenKind::RedirectIn,
         "<<<" => TokenKind::RedirectInStr,
         "<<" => TokenKind::RedirectInDoc,
         _ => unreachable!(),
      };

      let span = self.span();
      let token = Token { span, kind };
      token
   }

   pub fn token(&mut self) -> Token {
      let Some(char) = self.first() else {
         return Token {
            kind: TokenKind::Eof,
            span: Span::zero(self.pos),
         };
      };

      let token = match char {
         '(' => self.single_byte_token(TokenKind::OpenParen),
         ')' => self.single_byte_token(TokenKind::CloseParen),
         '$' => self.single_byte_token(TokenKind::Dollar),
         ';' => self.single_byte_token(TokenKind::Semi),
         '"' => self.single_byte_token(TokenKind::DoubleQuote),
         '\'' => self.single_byte_token(TokenKind::SingleQuote),
         '\n' => self.single_byte_token(TokenKind::Newline),
         ' ' => self.single_byte_token(TokenKind::WhiteSpace),
         ':' => self.single_byte_token(TokenKind::Colon),
         '|' => self.pipe(),
         '&' => self.ampersand(),
         '>' | '<' => self.redirect(),
         '\\' if self.second().is_none() => self.single_byte_token(TokenKind::Unknown),
         _ => self.shell_word(char),
      };

      token
   }
}


