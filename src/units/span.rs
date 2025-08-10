#[derive(Clone, Copy)]
pub struct TokenSpan {
    pub start: usize,
    pub end: usize,
    pub line_num: usize,
    pub column_range: ColumnRange,
}

impl TokenSpan {
    pub fn consume_char(&mut self, length: usize) {
        self.end += length;
        self.column_range.inc();
    }

    pub fn parsed_token(&mut self) -> Self {
        let output = *self;
        self.start = self.end;
        self.column_range.reset();
        output
    }

    pub fn skip(&mut self, offset: usize) {
        self.end += offset;
        self.start = self.end;
    } 

}

impl Default for TokenSpan {
    fn default() -> Self {
        Self {
            start: 0,
            end: 0,
            line_num: 1,
            column_range: ColumnRange::default(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct ColumnRange {
    pub start: usize,
    pub end: usize,
}

impl ColumnRange {
    pub fn reset(&mut self) {
        self.start = 1;
        self.end = 1;
    }

    pub fn inc(&mut self) {
        self.end += 1;
    }
}

impl Default for ColumnRange {
    fn default() -> Self {
        Self { start: 1, end: 1 }
    }
}

#[derive(Clone, Copy)]
pub struct LineByteRange {
    start: usize,
    end: usize,
}

impl LineByteRange {
    pub fn parsed_newline(&mut self, offset: usize) -> Self {
        static LENGTH_OF_NEWLINE: usize = '\n'.len_utf8();
        self.end = offset - LENGTH_OF_NEWLINE;
        let output = *self;

        self.end = offset;
        self.start = self.end;

        output
    }
}

impl Default for LineByteRange {
    fn default() -> Self {
        Self { start: 0, end: 0 }
    }
}
