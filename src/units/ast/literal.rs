use std::fmt::{Display, Formatter, self};

#[derive(Debug, Eq, PartialEq)]
pub enum Literal {
    Number(Box<str>),
}


impl Display for Literal {
    fn fmt (&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Number(number) => number,
        };
        write!(f, "{}", s)
    }
}
