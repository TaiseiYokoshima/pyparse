use std::fmt;
use std::ops::Range;

#[derive(Debug)]
pub enum NumberError {
    MultipleDots(usize),
    TooManyDigits,
    InvalidChar(usize),
}

impl fmt::Display for NumberError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MultipleDots(_) => write!(f, "number literal can only contain a single '.'"),
            Self::InvalidChar(_) => writeln!(f, "number literal can only contain digits or a '.'"),
            Self::TooManyDigits => writeln!(f, "the literal contains too many digits"),
        }
    }
}
