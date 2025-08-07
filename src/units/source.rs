use std::{collections::VecDeque};
use std::ops::{Index, Range};

pub struct Source {
    src: String,
    lines: VecDeque<usize>,
    // current_line: usize,
    // current_column: usize,
}

impl Source {
    fn new(src: String) -> Self {
        Self {
            src,
            lines: VecDeque::default(),
            // current_line: 0,
            // current_column: 0,
        }
    }

    fn next_line(&mut self, byte_offset: usize) {
        self.lines.push_back(byte_offset);
    }
}


// impl<'src> Index<Range<usize>> for Source {
//     type Output = &'src str;
//
//     fn index(&self, index: Range<usize>) -> &'src Self::Output {
//         
//     }
// }
