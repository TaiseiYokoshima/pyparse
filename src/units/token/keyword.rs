use std::fmt::{Display, Formatter, Result};

use strum_macros::{EnumString, EnumVariantNames};

#[derive(Debug, EnumString, EnumVariantNames, PartialEq, Eq)]
pub enum Keyword {
    #[strum(serialize = "def")]
    Def,

    #[strum(serialize = "not")]
    Not,

    #[strum(serialize = "and")]
    And,
}

impl Display for Keyword {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Def => write!(f, "Key(def)"),
            Self::Not => write!(f, "Key(not)"),
            Self::And => write!(f, "Key(and)"),
        }
    }
}
