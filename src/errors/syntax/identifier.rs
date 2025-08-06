use std::fmt;

#[derive(Debug)]
pub enum IdentError<'src> {
    DoesNotStartWithLetter(&'src str),
    InvalidChar(&'src str),
}

impl<'src> fmt::Display for IdentError<'src> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidChar(char) => write!(f, "'{}' is not a valid charater for identifiers. Identifiers must only consist of letters, digits, and a '_' (underscore)", char), 
            Self::DoesNotStartWithLetter(_) => write!(f, "Identifier must start with a letter or a '_' (underscore)"), 
        }
    }
}


