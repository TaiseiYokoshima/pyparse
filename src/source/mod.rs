use std::collections::VecDeque;
use std::ops::{Index, Range, RangeFrom};
use std::str::Chars;

#[derive(Debug)]
pub struct Source {
    src: String,
    line_ranges: VecDeque<Range<usize>>,
}


impl Source {
    pub fn new(src: String) -> Self {
        let mut line_ranges = VecDeque::new();

        let mut start = 0;
        let mut index = 0;

        for char in src.chars() {
            index += char.len_utf8();

            if char == '\n' {
                line_ranges.push_back(Range { start, end: index });
                start = index;
            };

        }

        Self { src, line_ranges }
    }


    pub fn line_range(&self, range: &Range<usize>) -> (usize, Range<usize>) {
        let start = range.start;
        let end = range.end;

        // println!("got range: {:?}", range);

        for (number, line) in self.line_ranges.iter().enumerate() {
            if start <= line.end && end <= line.end {
                return (number + 1, line.clone());
            };
        };

        panic!("for some reason did not find the range")
    }


    pub fn line_str(&self, range: &Range<usize>) -> String {
        let mut string = String::from(&self.src[range.start..range.end - 1]);
        string.push(' ');
        string
    }


    pub fn print_lines(&self) {
        for (number, line) in self.line_ranges.iter().enumerate() {
            println!("{}| {:?}", number + 1, &self.src[line.clone()])
        }
    }
}


impl<'src> IntoIterator for &'src Source {
    type Item = char;
    type IntoIter = Chars<'src>;

    fn into_iter(self) -> Self::IntoIter {
        self.src.chars()
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
    fn index(&self, index: RangeFrom<usize>) -> &Self::Output {
        &self.src[index]
    }
}
