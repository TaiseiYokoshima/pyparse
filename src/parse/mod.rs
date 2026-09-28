// mod parser;
mod language;
// mod shell;

#[derive(Debug, Clone, Copy)]
pub enum ErrorKind {
   ExpectedExpression,
   ExpectedOperator,
   ExpectedCloseParen,
}
