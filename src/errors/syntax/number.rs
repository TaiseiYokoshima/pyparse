use std::fmt;

#[derive(Debug)]
pub enum NumberError<'src> {
    MultipleDots(&'src str),
    TooManyDigits(&'src str),
    InvalidChars(&'src str),
}

impl<'src> fmt::Display for NumberError<'src> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MultipleDots(_) => write!(f, "number literal can only contain a single '.'"),
            Self::InvalidChars(_) => writeln!(f, "number literal can only contain digits or a '.'"),
            Self::TooManyDigits(_) => writeln!(f, "the literal contains too many digits"),
        }
    }
}
