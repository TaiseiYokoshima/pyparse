use std::collections::VecDeque;
use std::ops::{Index, Range, RangeFrom};

pub struct Source {
    pub src: String,
    pub lines: VecDeque<usize>,
    // current_line: usize,
    // current_column: usize,
}

impl Source {
    pub fn new(src: String) -> Self {
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

impl AsRef<String> for Source {
    fn as_ref(&self) -> &String {
        &self.src
    }
}

impl Index<Range<usize>> for Source {
    type Output = str;

    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.src[index]
    }
}

impl Index<RangeFrom<usize>> for Source {
    type Output = str;

    fn index<'src>(&'src self, index: RangeFrom<usize>) -> &'src Self::Output {
        let string = &self.src;
        &string[index]
    }
}


