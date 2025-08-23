use crate::{lex::TokenKind, source::Source};
use std::fmt::Write;

use super::{ErrorKind, ParserError};
use std::{collections::VecDeque, ops::Range};


pub struct Diagnostics<'src> {
    pub errors: VecDeque<ParserError>,
    src: &'src Source,
}

impl<'src> Diagnostics<'src> {
    pub fn new(errors: VecDeque<ParserError>, src: &'src Source) -> Self {
        Self { errors, src, }
    }


    fn build_highlighter(line_string: &str, range: Range<usize>) -> (usize, usize, String) {
        let start = range.start;
        let end = range.end;


        let mut output = Vec::new();
        let mut index = 0;
        let mut column_start = 0;
        let mut column_count = 0;

        for (column, char) in line_string.chars().enumerate() {
            if index >= start && index < end {
                match output.get(output.len() - 1) {
                    Some('^') => {
                        output.pop();
                        output.push('-');
                        output.push('-');
                    },
                    Some('-') => output.push('-'),
                    None | Some(' ') => {
                        output.push('^');
                        column_start = column + 1;
                    },
                    what => panic!("got {:?}", what),
                };

                column_count += 1;
            } else {
                output.push(' ');
            };

            index += char.len_utf8();
        };

        (column_start, column_count, output.iter().collect())
    }


    fn normalize_range_to_line(line: &Range<usize>, error: &Range<usize>) -> Range<usize> {
        let start = error.start - line.start;
        let end = error.end - line.start;

        Range {
            start,
            end,
        }
    }


    fn second_message(error: &ParserError) -> String {
        let first_bit = "but found ".to_string();

        match error.token {
            TokenKind::Eof | TokenKind::Newline | TokenKind::None => {
                first_bit + "nothing"
            },
            kind => first_bit + &kind.to_string(),
        }
    }

    
    fn messages(error: &ParserError) -> (String, String) {
        let (first, second) = match error.error {
            ErrorKind::ExpectedOperator => {
                let first = "expected one of `+`, `-`, `*`, `/`, `%`), `.`,".to_string(); 
                let second = Self::second_message(error);
                (first, second)
            }
            ErrorKind::ExpectedExpression => {
                let first = "expected an expression".to_string();
                let second = Self::second_message(error);
                (first, second)
            }
            ErrorKind::ExpectedCloseParen => {
                let first = "could not find the closing parenthesis".to_string();
                let second = "this parenthesis is not closed".to_string();
                (first, second)
            }
        };

        (first, second)
    }

        


    pub fn report(&self, error: &ParserError, index: usize) {
        let (number, line) = self.src.line_range(&error.range);
        let line_string = self.src.line_str(&line);
        let (column_start, column_count, highlight) = {
            let error_range = Self::normalize_range_to_line(&line, &error.range);
            Self::build_highlighter(&line_string, error_range)
        };

        let (first_message, second_message) = Self::messages(error);



        let mut output = String::new();

        writeln!(output, "Error {}: {}", index, first_message).expect("Should not have errored here");
        write!(output, "File:{}:{}", number, column_start).expect("Should not have errored here");

        if column_count > 1 {
            write!(output, ":{}", column_start + column_count - 1).expect("Should not have errored here");
        };

        writeln!(output).expect("Should not have errored here");
        writeln!(output, "{}", line_string).expect("Should not have errored here");
        writeln!(output, "{}", highlight).expect("Should not have errored here");
        writeln!(output, "{}", second_message).expect("Should not have errored here");

        print!("{}", output);

        
    }

    pub fn report_all(&self) {
        let mut index = 0;
        for error in &self.errors {
            self.report(&error, index + 1);
            index += 1;
        }
    }
}
