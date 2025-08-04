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



