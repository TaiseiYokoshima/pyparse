use std::fmt;

use super::{identifier::IdentError, NumberError};

#[derive(Debug)]
pub enum SyntaxError<'src> {
    NumberError(NumberError<'src>),
    IdentifierError(IdentError<'src>),
    UnexpectedToken(&'src str),
}

impl<'src> fmt::Display for SyntaxError<'src> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedToken(token) => writeln!(f, "unexpected token \"{}\"", token),
            Self::IdentifierError(char) => {
                writeln!(f, "found an invalid character '{}' in an identifier", char)
            }
            Self::NumberError(literal_err) => writeln!(f, "{}", literal_err),
        }
    }
}


impl<'src> From<NumberError<'src>> for SyntaxError<'src> {
    fn from(value: NumberError<'src>) -> SyntaxError<'src> {
        Self::NumberError(value)
    }
}
impl<'src> From<IdentError<'src>> for SyntaxError<'src> {
    fn from(value: IdentError<'src>) -> SyntaxError<'src> {
        Self::IdentifierError(value)
    }
}
