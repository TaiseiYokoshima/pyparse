use std::collections::VecDeque;
use std::fmt::{self, Write};

use crate::lex::{Lexer, Token, TokenKind};
use crate::source::Source;

#[derive(Debug)]
pub struct Tokens(VecDeque<Token>);
impl From<Lexer<'_>> for Tokens {
   fn from(value: Lexer<'_>) -> Self {
      Self(value.tokens)
   }
}

impl fmt::Display for Tokens {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "Tokens [\n")?;
      for token in &self.0 {
         write!(f, "{}\n", token)?;
      }

      write!(f, "]")
   }
}

#[derive(Debug)]
pub struct TokenStream<'src> {
   src: &'src Source,
   pub stream: VecDeque<Token>,
}

impl<'src> From<Lexer<'src>> for TokenStream<'src> {
   fn from(value: Lexer<'src>) -> Self {
      Self {
         src: value.src,
         stream: value.tokens,
      }
   }
}

impl<'src> fmt::Display for TokenStream<'src> {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      let src = self.src;
      let mut s = String::new();
      write!(s, "Tokens: [")?;

      for token in &self.stream {
         let len = token.size;
         write!(s, "{} ", token)?;
      }

      s.pop();
      write!(s, " ]")?;
      writeln!(f, "{}", s)
   }
}

impl<'src> Into<(&'src Source, VecDeque<Token>)> for TokenStream<'src> {
   fn into(self) -> (&'src Source, VecDeque<Token>) {
      (self.src, self.stream)
   }
}
