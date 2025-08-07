use std::fmt;

use super::{NumberError, identifier::IdentError};
use crate::SrcSpan;

#[derive(Debug)]
pub enum ErrorKind<'src> {
    NumberError(NumberError<'src>),
    IdentifierError(IdentError<'src>),
    UnexpectedToken,
}

#[derive(Debug)]
pub struct SyntaxError<'src> {
    span: SrcSpan<'src>,
    kind: ErrorKind<'src>,
}

impl<'src> SyntaxError<'src> {
    pub fn new(span: SrcSpan<'src>, kind: ErrorKind<'src>) -> Self {
        Self { span, kind }
    }
}

impl<'src> fmt::Display for SyntaxError<'src> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ErrorKind::UnexpectedToken => writeln!(f, "unexpected token \"{}\"", self.span),
            ErrorKind::IdentifierError(char) => {
                writeln!(f, "found an invalid character '{}' in an identifier", char)
            }
            ErrorKind::NumberError(literal_err) => writeln!(f, "{}", literal_err),
        }
    }
}

// impl<'src> From<NumberError<'src>> for SyntaxError<'src> {
//     fn from(value: NumberError<'src>) -> SyntaxError<'src> {
//         Self::NumberError(value)
//     }
// }
// impl<'src> From<IdentError<'src>> for SyntaxError<'src> {
//     fn from(value: IdentError<'src>) -> SyntaxError<'src> {
//         Self::IdentifierError(value)
//     }
// }
