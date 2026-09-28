// mod stream;
mod token;

mod shell;
mod language;

#[cfg(test)]
mod test;

// use crate::source::Source;
pub use token::{Span, Token, TokenKind};
pub use shell::ShellCursor;



use std::str::Chars as std_chars;

pub struct Chars<'src> {
   chars: std_chars<'src>,
}


impl<'src> Chars<'src> {
   pub fn new(str: &'src str) -> Self {
      Self { chars: str.chars() }
   }
}

impl<'src> From<&'src str> for Chars<'src> {
   fn from(value: &'src str) -> Self {
      Self::new(value)
   }
}

impl<'src> Iterator for Chars<'src> {
   type Item = char;

   fn next(&mut self) -> Option<Self::Item> {
      self.chars.next()
   }
}

static mut DEBUG: bool = false;

macro_rules! dprint {
   ($($arg:tt)*) => {
      unsafe { if DEBUG {
         println!($($arg)*);
      };};
   };
}


#[derive(Debug, PartialEq, Eq)]
pub enum LexErrorKind {
   TrailingBackslash,
   TrailingLang,
}

#[derive(Debug)]
pub struct LexError {
   kind: LexErrorKind,
   span: Span,
}
