use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Span {
   pub start: usize,
   pub len: u32,
}

impl Span {
   pub fn zero(pos: usize) -> Self {
      Self {
         start: pos,
         len: 0,
      }
   }

   pub fn one(pos: usize) -> Self {
      Self {
         start: pos,
         len: 1,
      }
   }

   pub fn new(pos: usize, len: u32) -> Self {
      Self {
         start: pos,
         len: len,
      }
   }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Token {
   pub kind: TokenKind,
   pub span: Span,
}

impl Token {
   pub fn new(kind: TokenKind, span: Span) -> Self {
      Self {
         kind, span
      }
   }
}


impl fmt::Display for Token {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      let len = self.span.len;

      match self.kind {
         TokenKind::Dot => write!(f, "Dot({})", len),
         TokenKind::Plus => write!(f, "Plus({})", len),
         TokenKind::Minus => write!(f, "Minus({})", len),
         TokenKind::Star => write!(f, "Star({})", len),
         TokenKind::Slash => write!(f, "Slash({})", len),
         TokenKind::Percent => write!(f, "Percent({})", len),
         TokenKind::OpenParen => write!(f, "OpenParen({})", len),
         TokenKind::CloseParen => write!(f, "CloseParen({})", len),
         TokenKind::Ident => write!(f, "Ident({})", len),
         TokenKind::WhiteSpace => write!(f, "WhiteSpace({})", len),
         TokenKind::InvalidChar => write!(f, "InvalidChar({})", len),
         TokenKind::Semi => write!(f, "Semi({})", len),
         TokenKind::Colon => write!(f, "Colon({})", len),
         TokenKind::Comma => write!(f, "Comma({})", len),
         TokenKind::Eof => write!(f, "Eof"),
         TokenKind::Ampersand => write!(f, "Ampersand({})", len),
         TokenKind::Dollar => write!(f, "Dollar({})", len),
         TokenKind::Pipe => write!(f, "Pipe({})", len),
         TokenKind::Newline => write!(f, "Newline({})", len),
         TokenKind::Word => write!(f, "ShellLiteral({})", len),
         TokenKind::Backslash => write!(f, "ShellEscape({})", len),

         TokenKind::RedirectOut => write!(f, "RedirectOut({})", len),
         TokenKind::RedirectOutAppend => write!(f, "RedirectOutAppend({})", len),
         TokenKind::RedirectIn => write!(f, "RedirectIn({})", len),
         TokenKind::RedirectInDoc => write!(f, "RedirectInDoc({})", len),
         TokenKind::RedirectInStr => write!(f, "RedirectInStr({})", len),

         TokenKind::DoubleQuote => write!(f, "ShellDoubleQuote({})", len),
         TokenKind::SingleQuote => write!(f, "ShellSingleQuote({})", len),
         TokenKind::Fd => write!(f, "Fd({})", len),

         TokenKind::And => write!(f, "And({})", len),
         TokenKind::Or => write!(f, "Or({})", len),

         TokenKind::Underscore => write!(f, "Underscore({})", len),

         TokenKind::Unknown => write!(f, "Unknown({})", len),
      }
   }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenKind {
   WhiteSpace,
   Newline,

   Dollar,
   Plus,
   Minus,
   Slash,
   Star,
   Percent,
   Underscore,

   // shared
   OpenParen,
   CloseParen,
   Ampersand,
   Pipe,
   Semi,
   Colon,
   Backslash,
   Or,
   And,

   // shell tokens
   Word,
   DoubleQuote,
   SingleQuote,
   Fd,

   // shell redirect
   RedirectOut,
   RedirectOutAppend,
   RedirectIn,
   RedirectInDoc,
   RedirectInStr,

   Unknown,

   Dot,
   Comma,

   Ident,
   InvalidChar,

   Eof,
}
