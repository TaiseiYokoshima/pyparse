use std::fmt::{Display, Formatter, self};

pub type BP = f32;

pub mod bp {
    use super::BP;
    use crate::tokenizer::{Operator, Token};

    pub const ADD: (BP, BP) = (1.0, 1.1);
    pub const SUB: (BP, BP) = (1.0, 1.1);
    pub const MUL: (BP, BP) = (1.0, 1.1);
    pub const DIV: (BP, BP) = (1.0, 1.1);

    pub const NEG: (BP, BP) = (10.0, 10.1);
    pub const NOT: (BP, BP) = (10.0, 10.1);


    pub const START: (BP, BP) = (0.0, 0.0) ;
    pub const NEWLINE: (BP, BP) = (0.0, 0.0) ;

    pub fn get_infix(op: &Operator) -> (BP, BP) {
        match op {
            Operator::Plus => ADD,
            Operator::Minus => SUB,
            Operator::Star => MUL,
            Operator::Slash => DIV,
            _ => panic!("you gave the wrong one")
        }
    }

    pub fn binds_left(left: BP, operator: &Operator) -> bool {
        left > get_infix(operator).0
    }
}


#[derive(Debug, Eq, PartialEq)]
pub struct Ident(Box<str>);
impl fmt::Display for Ident {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }

}

#[derive(Debug, Eq, PartialEq)]
pub enum BinOperator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
}

impl Display for BinOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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


#[derive(Debug, Eq, PartialEq)]
pub struct Assign {
    identifier: Ident,
    expression: Expr,
}


#[derive(Debug, Eq, PartialEq)]
pub struct BinOperation {
    op: BinOperator, 
    left: Expr,
    right: Expr
}

impl BinOperation {
    pub fn new(op: BinOperator, left: Expr, right: Expr) -> Self {
        Self {
            op,
            left,
            right
        }
    }
}

impl Display for BinOperation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "( {} {} {} )", self.left, self.op, self.right)

    }

}




#[derive(Debug, Eq, PartialEq)]
pub enum Literal {
    Number(Box<str>),
}


impl Display for Literal {
    fn fmt (&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Number(number) => number,
        };
        write!(f, "{}", s)
    }
}


#[derive(Debug, Eq, PartialEq)]
pub enum Expr {
    // Assign(Box<Assign>),
    BinOp(Box<BinOperation>),
    Ident(Ident),
    Literal(Literal),
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::BinOp(bin_op) => write!(f, "{}", bin_op),
            Self::Literal(literal) => write!(f, "{}", literal),
            Self::Ident(id) => write!(f, "{}", id),
        }
    }
}


#[derive(Debug, Eq, PartialEq, Default)]
pub struct Ast(Vec<Expr>);
