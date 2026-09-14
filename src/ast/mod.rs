mod expr;
mod command;

use crate::lex::TokenKind;

pub struct Span {
   pos: usize,
   len: usize,
}

pub struct Token {
   kind: TokenKind,
   span: Span,
}
