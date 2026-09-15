mod expr;
pub mod command;

pub use crate::lex::TokenKind;

pub struct Span {
   pub pos: usize,
   pub len: usize,
}

pub struct Token {
   pub kind: TokenKind,
   pub span: Span,
}
