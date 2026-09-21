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
