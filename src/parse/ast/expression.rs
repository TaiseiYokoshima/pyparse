use super::{BinOpn, Ident, Literal};
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Expr {
    // Assign(Box<Assign>),
    BinOp(Box<BinOpn>),
    Ident(Box<str>),
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
