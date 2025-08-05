use std::fmt::{Display, Formatter, Result};

use strum_macros::{EnumString, EnumVariantNames};

#[derive(Debug, EnumString, EnumVariantNames, PartialEq, Eq, Clone)]
pub enum Operator {
    #[strum(serialize = "+")]
    Plus,

    #[strum(serialize = "-")]
    Minus,

    #[strum(serialize = "*")]
    Star,

    #[strum(serialize = "/")]
    Slash,

    #[strum(serialize = "%")]
    Modulus,
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::Star => write!(f, "*"),
            Self::Slash => write!(f, "/"),
            Self::Modulus => write!(f, "%"),
        }
    }
}

// impl Into<BinOperator> for Operator  {
//     fn into(self) -> BinOperator {
//         match self {
//             Self::Plus => BinOperator::Add,
//             Self::Minus => BinOperator::Sub,
//             Self::Star => BinOperator::Mul,
//             Self::Slash => BinOperator::Div,
//             Self::Modulus => BinOperator::Mod,
//         }
//     }
// }
