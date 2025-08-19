use std::ops::Range;

use super::{BinOpn, Literal};

#[derive(Debug)]
pub enum Expr {
    BinOp(Box<BinOpn>),
    Ident(Range<usize>),
    Literal(Literal),
}

// impl Display for Expr {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//         match self {
//             Self::BinOp(bin_op) => write!(f, "{}", bin_op),
//             Self::Literal(literal) => write!(f, "{}", literal),
//             Self::Ident(id) => write!(f, "{}", id),
//         }
//     }
// }
