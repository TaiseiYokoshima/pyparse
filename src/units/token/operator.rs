use std::fmt::{Display, Formatter, Result};

use strum_macros::{EnumString, EnumVariantNames};

#[derive(Debug, EnumString, EnumVariantNames, PartialEq, Eq, Clone)]
pub enum Operator {
    #[strum(serialize = "+")]
    Add,

    #[strum(serialize = "-")]
    Sub,

    #[strum(serialize = "*")]
    Mul,

    #[strum(serialize = "/")]
    Div,

    #[strum(serialize = "%")]
    Mod,

    #[strum(serialize = "**")]
    Pow,

    #[strum(serialize = "//")]
    Flo,
}

impl Operator {
    pub fn new(src: &str) -> Option<Self> {
        match src {
            "+" => Some(Operator::Add),
            "-" => Some(Operator::Sub),
            "*" => Some(Operator::Mul),
            "/" => Some(Operator::Div),
            "%" => Some(Operator::Mod),
            "**" => Some(Operator::Pow),
            "//" => Some(Operator::Flo),
            _ => None,
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Add => write!(f, "+"),
            Self::Sub => write!(f, "-"),
            Self::Mul => write!(f, "*"),
            Self::Div => write!(f, "/"),
            Self::Mod => write!(f, "%"),
            Self::Pow => write!(f, "**"),
            Self::Flo => write!(f, "//"),
        }
    }
}
