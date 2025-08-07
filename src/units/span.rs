use std::fmt;

#[derive(Debug)]
pub struct SrcSpan<'src> {
    src: &'src String,
    start: usize,
    end: usize,
    line: usize, 
    column: usize,
}


impl<'src> fmt::Display for SrcSpan<'src> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.src[self.start..self.end])
    }
}


impl<'src> AsRef<str> for SrcSpan<'src> {
    fn as_ref(&self) -> &'src str {
        &self.src[self.start..self.end]
    }
}
