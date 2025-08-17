use std::fmt::{Result, Display, Formatter};


pub type BP = u8;


#[derive(Debug, Eq, PartialEq)]
pub enum BinOpr {
    Add,
    Sub,
    Mul,
    Div,
    Flo,
    Mod,
    Exp,
}


impl BinOpr {
    pub fn get_bp(&self) -> (BP, BP) {
        match self {
            Self::Add => (10, 11),
            Self::Sub => (10, 11),
            Self::Mul => (20, 21),
            Self::Div => (20, 21),
            Self::Flo => (20, 21),
            Self::Mod => (20, 21),
            Self::Exp => (31, 30),
        }
    }
}



impl Display for BinOpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let s = match self {
            Self::Add => "+",
            Self::Sub => "-",
            Self::Mul => "*",
            Self::Div => "/",
            Self::Mod => "%",
            Self::Exp => "**",
            Self::Flo => "//",
        };

        write!(f, "{}", s)
    }
}
