use std::collections::VecDeque;
use std::ops::Range;



#[derive(Debug)]
pub struct Source {
    src: String,
    line_ranges: VecDeque<Range<usize>>,
}


impl Source {

}

