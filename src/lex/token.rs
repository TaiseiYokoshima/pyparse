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
         TokenKind::Dot => write!(f, "Dot(:{})", size),
         TokenKind::Plus => write!(f, "Plus(:{})", size),
         TokenKind::Minus => write!(f, "Minus(:{})", size),
         TokenKind::Star => write!(f, "Star(:{})", size),
         TokenKind::Slash => write!(f, "Slash(:{})", size),
         TokenKind::Percent => write!(f, "Percent(:{})", size),
         TokenKind::OpenParen => write!(f, "OpenParen(:{})", size),
         TokenKind::CloseParen => write!(f, "CloseParen(:{})", size),
         TokenKind::Ident => write!(f, "Ident({})", size),
         TokenKind::WhiteSpace => write!(f, "WhiteSpace(:{})", size),
         TokenKind::InvalidChar => write!(f, "InvalidChar(:{})", size),
         TokenKind::Semi => write!(f, "Semi(:{})", size),
         TokenKind::Colon => write!(f, "Colon(:{})", size),
         TokenKind::Comma => write!(f, "Comma(:{})", size),
         TokenKind::Eof => write!(f, "Eof"),
         TokenKind::DoubleQuote => write!(f, "DoubleQuote(:{})", size),
         TokenKind::SingleQuote => write!(f, "SingleQuote(:{})", size),
         TokenKind::Ampersand => write!(f, "Ampersand(:{})", size),
         TokenKind::Dollar => write!(f, "Dollar(:{})", size),
         TokenKind::Pipe => write!(f, "Pipe(:{})", size),
         TokenKind::Newline => write!(f, "Newline(:{})", size),
         TokenKind::OpenAngle => write!(f, r"OpenAngle(:{})", size),
         TokenKind::CloseAngler => write!(f, r"CloseAngler(:{})", size),

      }
   }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenKind {
   WhiteSpace,
   Newline,

   OpenParen,
   CloseParen,

   Dollar,
   Pipe,
   Ampersand,

   Plus,
   Minus,
   Slash,
   Star,
   Percent,

   OpenAngle,
   CloseAngler,

   

   Literal,
   Var,
   ShellSubStart,
   ShellSubStart,
   ShellVar,





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


