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
