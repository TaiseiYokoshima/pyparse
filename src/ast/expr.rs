use std::collections::VecDeque;

use crate::ast::Span;

pub enum BinOperator {
   Add,
   Min,
   Sub,
   Div
}

pub enum Lit {
   Str,
   Num,
   Bool,
}

pub struct Expr {
   kind: ExprKind,
   span: Span,
}

pub struct Block { 
   stmts: VecDeque<Stmt>,
   span: Span,
}

pub struct If {
   condition: Expr,
   first_block: Block,
   elifs: VecDeque<(Expr, Block)>,
   else_: Option<Block>,
}

pub enum ExprKind {
   BinOp(BinOperator, Box<Expr>, Box<Expr>),
   Lit(Lit),
   If(Box<Block>),
   Block(Box<Block>),
   Id(Span),
}

pub enum StmtKind {
   Expr(Expr),
   ReturnExpr(Expr),
   Def(Span, Block),
   Let(Span, Expr),
   Export(VecDeque<(Span, Option<Span>)>),
   // Cmd(Cmd),
}

pub struct Stmt {
   kind: StmtKind,
   span: Span,
}
