use crate::units::Operator;
use std::fmt::{Result, Display, Formatter};

#[derive(Debug, Eq, PartialEq)]
pub enum BinOpr {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Flor,
}

impl Display for BinOpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let s = match self {
            Self::Add => "+",
            Self::Sub => "-",
            Self::Mul => "*",
            Self::Div => "/",
            Self::Mod => "%",
            Self::Pow => "**",
            Self::Flor => "//",
        };

        write!(f, "{}", s)
    }
}

impl From<Operator> for BinOpr {
    fn from(op: Operator) -> BinOpr {
        match op {
            Operator::Add => BinOpr::Add,
            Operator::Sub => BinOpr::Sub,
            Operator::Mul => BinOpr::Mul,
            Operator::Div => BinOpr::Div,
            Operator::Mod => BinOpr::Mod,
            Operator::Pow => BinOpr::Pow,
            Operator::Flo => BinOpr::Flor,
        }
    }
}
