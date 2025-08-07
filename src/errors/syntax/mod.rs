mod syntax;

mod number;
mod identifier;


pub use syntax::{SyntaxError, ErrorKind};
pub use number::NumberError;
pub use identifier::IdentError;






