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

    pub fn none() -> Self {
        Self {
            kind: TokenKind::None,
            size: 0,
        }
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
            TokenKind::Number => write!(f, "Number : {}", size),
            TokenKind::Newline => write!(f, "Newline : {}", size),
            TokenKind::Ident => write!(f, "Ident : {}", size),
            TokenKind::WhiteSpace => write!(f, "WhiteSpace : {}", size),
            TokenKind::InvalidChar => write!(f, "Invalid Char : {}", size),
            TokenKind::Semi => write!(f, "Semi : {}", size),
            TokenKind::Colon => write!(f, "Colon : {}", size),
            TokenKind::Comma => write!(f, "Comma : {}", size),
            TokenKind::Eof => write!(f, "End : 0"),
            TokenKind::None => write!(f, "None : 0"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenKind {
    WhiteSpace,
    Newline,

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

    Ident,
    Number,
    InvalidChar,


    // for error reporting purpose only
    None,
    Eof,
}
