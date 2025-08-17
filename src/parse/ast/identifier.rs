use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Eq, PartialEq)]
pub struct Ident(Box<str>);
impl Display for Ident {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.0)
    }
}
