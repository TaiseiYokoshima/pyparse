use std::fmt::{Display, Formatter, Result};

use crate::units::{BinOpn, Ident, Literal};

#[derive(Debug, Eq, PartialEq)]
pub enum Expr {
    // Assign(Box<Assign>),
    BinOp(Box<BinOpn>),
    Ident(Ident),
    Literal(Literal),
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::BinOp(bin_op) => write!(f, "{}", bin_op),
            Self::Literal(literal) => write!(f, "{}", literal),
            Self::Ident(id) => write!(f, "{}", id),
        }
    }
}
