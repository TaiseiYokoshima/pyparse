use crate::lex::{ShellCursor, Token, TokenKind};
use crate::ast::command::{Cmd, Redirect, Word, WordPart, WordPartKind};

macro_rules! redirects_arm {
   () => {
      TokenKind::RedirectIn | TokenKind::RedirectInDoc | TokenKind::RedirectInStr | TokenKind::RedirectOut | TokenKind::RedirectOut
   };
}

pub struct ShellParser<'src> {
   builder: Vec<Token>,
   src: &'src str,
   lexer: ShellCursor<'src>,
}


impl<'src> ShellParser<'src> {
   pub fn new (src: &'src str) -> Self {
      let cursor = ShellCursor::new(src);
      let builder = vec![];
      Self {
         lexer: cursor, src, builder
      }
   }

   pub fn token(&mut self) -> Token {
      self.lexer.token()
   }

   fn word(&mut self, words: &mut Vec<Word>, token: Token) {
      let mut token = token;
      loop {

         let kind = match token.kind {
            TokenKind::Word => WordPart::new(WordPartKind::StrLit, token.span),
            TokenKind::Colon => self.language(token),
            _ => todo!(),
         };



         let next = self.token();
         match next.kind {
            TokenKind::Word => token = next,
            _ => break,
         };
      };
   }

   fn redirect(&mut self, redirects: &mut Vec<Redirect>) {
      todo!()
   }

   fn language(&mut self, token: Token) -> WordPart {
      todo!()
   }

   fn command(&mut self) -> Option<Cmd> {
      let mut token = if let token = self.token() && token.kind == TokenKind::Eof {
         token
      } else {
         return None;
      };

      let mut words = vec![];
      let mut redirects = vec![];

      let bg= loop {
         match token.kind {
            TokenKind::Newline | TokenKind::Semi => break false,
            TokenKind::Ampersand => break true,
            TokenKind::Word => self.word(&mut words, token),
            redirects_arm!() => self.redirect(&mut redirects),
            TokenKind::Colon => self.language(),
            TokenKind::WhiteSpace => (),
            _ => todo!(),
         };
      };

      todo!()
   }


   pub fn parse(&mut self) -> Vec<Cmd> {
      todo!()
   }
}
