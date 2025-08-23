use std::ops::Range;
use std::fmt;

use super::{BinOpn, LiteralKind};

#[derive(Debug)]
pub enum Expr {
    BinOp(Box<BinOpn>),
    Ident(Range<usize>),
    Literal {
        kind: LiteralKind,
        range: Range<usize>,
    },
}

impl Expr {
    pub fn literal(kind: LiteralKind, range: Range<usize>) -> Self {
        Self::Literal { kind, range }
    }

    pub fn bin_op(bin_op: BinOpn) -> Self {
        Self::BinOp(Box::new(bin_op))
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BinOp(bin_op) => write!(f, "{}", bin_op),
            Self::Literal { kind, range } => write!(f, "{:?}", kind),
            Self::Ident(id) => write!(f, "Ident"),
        }
    }
}
