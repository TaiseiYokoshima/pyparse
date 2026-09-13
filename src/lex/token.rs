use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Token {
   pub kind: TokenKind,
   pub size: usize,
}

impl Token {
   pub fn new(kind: TokenKind, size: usize) -> Self {
      Self { kind, size }
   }
}

impl fmt::Display for Token {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      let size = self.size;

      match self.kind {
         TokenKind::Dot => write!(f, "'.' : {}", size),
         TokenKind::Plus => write!(f, "'+' : {}", size),
         TokenKind::Minus => write!(f, "'-' : {}", size),
         TokenKind::Star => write!(f, "'*' : {}", size),
         TokenKind::Slash => write!(f, "'/' : {}", size),
         TokenKind::Percent => write!(f, "'%' : {}", size),
         TokenKind::OpenParen => write!(f, "'(' : {}", size),
         TokenKind::CloseParen => write!(f, "')' : {}", size),
         TokenKind::Ident => write!(f, "Ident : {}", size),
         TokenKind::WhiteSpace => write!(f, "WhiteSpace : {}", size),
         TokenKind::InvalidChar => write!(f, "Invalid Char : {}", size),
         TokenKind::Semi => write!(f, "Semi : {}", size),
         TokenKind::Colon => write!(f, "Colon : {}", size),
         TokenKind::Comma => write!(f, "Comma : {}", size),
         TokenKind::Eof => write!(f, "End : 0"),
         TokenKind::DoubleQuote => write!(f, "DoubleQuote"),
         TokenKind::SingleQuote => write!(f, "SingleQuote"),
      }
   }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenKind {
   WhiteSpace,

   OpenParen,
   CloseParen,

   Plus,
   Minus,
   Slash,
   Star,
   Percent,

   Dot,
   Comma,
   Semi,
   Colon,

   DoubleQuote,
   SingleQuote,

   Ident,
   InvalidChar,

   Eof,
}

impl fmt::Display for TokenKind {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      match self {
         TokenKind::Dot => write!(f, "`.`"),
         TokenKind::Plus => write!(f, "`+`"),
         TokenKind::Minus => write!(f, "`-`"),
         TokenKind::Star => write!(f, "`*`"),
         TokenKind::Slash => write!(f, "`/`"),
         TokenKind::Percent => write!(f, "`%`"),
         TokenKind::OpenParen => write!(f, "`(`"),
         TokenKind::CloseParen => write!(f, "`)`"),
         TokenKind::Ident => write!(f, "Ident"),
         TokenKind::WhiteSpace => write!(f, "WhiteSpace"),
         TokenKind::InvalidChar => write!(f, "Invalid Char"),
         TokenKind::Semi => write!(f, "Semi"),
         TokenKind::Colon => write!(f, "Colon"),
         TokenKind::Comma => write!(f, "Comma"),
         TokenKind::Eof => write!(f, "End"),
         TokenKind::DoubleQuote => write!(f, "DoubleQuote"),
         TokenKind::SingleQuote => write!(f, "SingleQuote"),
      }
   }
}
