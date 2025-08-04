use std::fmt::{self, Display, Formatter};

use super::BinOpr;
use super::Expr;

#[derive(Debug, Eq, PartialEq)]
pub struct BinOpn {
    op: BinOpr,
    left: Expr,
    right: Expr,
}

impl BinOpn {
    pub fn new(op: BinOpr, left: Expr, right: Expr) -> Self {
        Self { op, left, right }
    }
}

impl Display for BinOpn {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "( {} {} {} )", self.left, self.op, self.right)
    }
}
