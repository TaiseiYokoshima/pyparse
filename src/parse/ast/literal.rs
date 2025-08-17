use std::fmt::{Display, Formatter, self};

#[derive(Debug)]
pub enum Literal {
    Number(f32),
}

// impl Eq for Literal {
//
// }


impl Display for Literal {
    fn fmt (&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Number(number) => number,
        };
        write!(f, "{}", s)
    }
}
