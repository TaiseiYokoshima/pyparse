use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Span {
   pub start: usize,
   pub end: usize,
}

impl Span {
   pub fn zero(pos: usize) -> Self {
      Self {
         start: pos,
         end: pos,
      }
   }

   pub fn one(pos: usize) -> Self {
      Self {
         start: pos,
         end: pos + 1,
      }
   }

   pub fn new(pos: usize, len: usize) -> Self {
      Self {
         start: pos,
         end: pos + len,
      }
   }
}

impl Span {
   pub fn len(&self) -> usize {
      self.end - self.start
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
      let len = self.span.len();

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
      }
   }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenKind {
   WhiteSpace,
   Newline,
   LineContinuation,

   Dollar,
   Plus,
   Minus,
   Slash,
   Star,
   Percent,

   // shared
   OpenParen,
   CloseParen,
   Ampersand,
   Pipe,
   Semi,
   Colon,
   Backslash,

   // shell tokens
   Word,
   DoubleQuote,
   SingleQuote,

   // shell redirect
   RedirectOut,
   RedirectOutAppend,
   RedirectIn,
   RedirectInDoc,
   RedirectInStr,




   Dot,
   Comma,

   Ident,
   InvalidChar,

   Eof,
}
