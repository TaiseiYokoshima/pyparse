use std::ops::Range;


#[derive(Debug)]
pub enum LiteralKind {
    Number,
}

#[derive(Debug)]
pub struct Literal {
    kind: LiteralKind,
    range: Range<usize>,
}

impl Literal {
    pub fn new(kind: LiteralKind, range: Range<usize>) -> Self {
        Literal {
            kind,
            range
        }
    }
}

// impl Display for Literal {
//     fn fmt (&self, f: &mut Formatter<'_>) -> fmt::Result {
//         match self {
//             Self::Number(option) => match option { 
//                 Some(number) => write!(f, "Number({})", number),
//                 None => write!(f, "Number(None)"),
//             },
//         }
//     }
// }
