use std::{fmt, ops::Range};

use super::{NumberError, identifier::IdentError};
use crate::Span;

#[derive(Debug)]
pub enum ErrorKind {
    NumberError(NumberError),
    IdentifierError(Range<usize>),
    UnexpectedToken,
    InvalidChar,
}

#[derive(Debug)]
pub struct SyntaxError {
    span: Span,
    kind: ErrorKind,
}

impl SyntaxError {
    pub fn new(span: Span, kind: ErrorKind) -> Self {
        Self { span, kind }
    }
}

// impl fmt::Display for SyntaxError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match &self.kind {
//             ErrorKind::UnexpectedToken => writeln!(f, "unexpected token \"{}\"", self.span),
//             ErrorKind::IdentifierError(_) => {
//                 writeln!(f, "found an invalid character in an identifier")
//             }
//             ErrorKind::NumberError(literal_err) => writeln!(f, "{}", literal_err),
//         }
//     }
// }

