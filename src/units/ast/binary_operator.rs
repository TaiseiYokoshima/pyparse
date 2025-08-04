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
        };

        write!(f, "{}", s)
    }
}

impl From<Operator> for BinOpr {
    fn from(op: Operator) -> BinOpr {
        match op {
            Operator::Plus => BinOpr::Add,
            Operator::Minus => BinOpr::Sub,
            Operator::Star => BinOpr::Mul,
            Operator::Slash => BinOpr::Div,
            Operator::Modulus => BinOpr::Mod,
        }
    }
}
