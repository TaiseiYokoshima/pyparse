mod ast;
mod parser;

use ast::{Assign, Ast, BP, BinOperation, Ident, Literal, bp};

pub use ast::{BinOperator, Expr};
pub use parser::Parser;
