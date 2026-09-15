use std::collections::VecDeque;
use std::str::Chars;

use crate::lex::{Token, TokenKind};

static mut DEBUG: bool = false;

macro_rules! dprint {
   ($($arg:tt)*) => {
      unsafe { if DEBUG {
         println!($($arg)*);
      };};
   };
}

use crate::source::Source;

pub struct Lexer<'src> {
   pub src: &'src Source,
   pub tokens: VecDeque<Token>,
   it: Chars<'src>,
   temp_char: Option<char>,
}

impl<'src> Lexer<'src> {
   pub fn new(src: &'src Source) -> Lexer<'src> {
      let it = src.into_iter();
      let tokens = VecDeque::default();
      let temp_char = None;

      Self {
         src,
         it,
         tokens,
         temp_char,
      }
   }

   #[inline]
   fn push(&mut self, kind: TokenKind, size: usize) {
      let token = Token::new(kind, size);
      dprint!("Token: {}", &token);
      self.tokens.push_back(token);
   }

   #[inline]
   fn advance(&mut self) -> Option<char> {
      if let Some(temp_char) = self.temp_char {
         dprint!("got temp: {}", temp_char);
         self.temp_char = None;
         return Some(temp_char);
      };

      dprint!("going to consuem next char");
      self.it.next()
   }

   #[inline]
   fn set_temp(&mut self, temp_char: char) {
      self.temp_char = Some(temp_char);
      dprint!("set temp: {:?}", temp_char);
   }

   fn parse_ident(&mut self) {
      let mut size = 1;
      while let Some(char) = self.advance() {
         match char {
            'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => {
               size += 1;
            }
            '.' => {
               self.push(TokenKind::Ident, size);
               self.push(TokenKind::Dot, 1);
               return;
            }

            '\n' | ' ' | '(' | ')' | '+' | '-' | '*' | '/' | '%' => {
               self.push(TokenKind::Ident, size);
               self.set_temp(char);
               return;
            }
            _ => {
               size += char.len_utf8();
            }
         };
      }
   }

   fn parse_whitespace(&mut self) {
      let mut size = 1;

      while let Some(char) = self.advance() {
         match char {
            ' ' | '\t' | '\r' => {
               size += 1;
            }

            char => {
               self.push(TokenKind::WhiteSpace, size);
               self.set_temp(char);
               return;
            }
         };
      }
   }

   fn parse_char(&mut self, first: char) {
      match first {
         ' ' | '\t' | '\r' => self.parse_whitespace(),
         '\n' => self.push(TokenKind::Newline, 1),

         '(' => self.push(TokenKind::OpenParen, 1),
         ')' => self.push(TokenKind::CloseParen, 1),

         '+' => self.push(TokenKind::Plus, 1),
         '-' => self.push(TokenKind::Minus, 1),
         '*' => self.push(TokenKind::Star, 1),
         '/' => self.push(TokenKind::Slash, 1),
         '%' => self.push(TokenKind::Percent, 1),

         '.' => self.push(TokenKind::Dot, 1),
         ',' => self.push(TokenKind::Comma, 1),
         ';' => self.push(TokenKind::Semi, 1),
         ':' => self.push(TokenKind::Colon, 1),

         '"' => self.push(TokenKind::DoubleQuote, 1),
         '\'' => self.push(TokenKind::SingleQuote, 1),

         'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => self.parse_ident(),
         _ => self.push(TokenKind::InvalidChar, first.len_utf8()),
      };
   }

   fn eof(&mut self) {
      let token = Token {
         size: 0,
         kind: TokenKind::Eof,
      };
      self.tokens.push_back(token);
   }

   pub fn tokenize<Output: From<Lexer<'src>>>(mut self, debug: bool) -> Output {
      unsafe { DEBUG = debug };

      while let Some(char) = self.advance() {
         self.parse_char(char);
      }

      self.eof();
      Output::from(self)
   }
}
