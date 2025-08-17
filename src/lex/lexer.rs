use std::collections::VecDeque;
use std::str::Chars;

use crate::lex::{Token, TokenKind};

pub struct Lexer<'src> {
    pub src: &'src str,
    pub tokens: VecDeque<Token>,
    it: Chars<'src>,
    temp_char: Option<char>,
    debug: bool,
}

impl<'src> Lexer<'src> {
    pub fn new(src: &'src str) -> Lexer<'src> {
        let it = src.chars();
        let tokens = VecDeque::default();
        let temp_char = None;

        Self {
            src,
            it,
            tokens,
            temp_char,
            debug: false,
        }
    }

    #[inline]
    fn push(&mut self, kind: TokenKind, len: usize) {
        let token = Token::new(kind, len);
        if self.debug {
            println!("Token: {}", &token)
        };
        self.tokens.push_back(token);
    }


    #[inline]
    fn next(&mut self) -> Option<char> {
        if let Some(temp_char) = self.temp_char {
            if self.debug { println!("got temp: {}", temp_char) };;
            self.temp_char = None;
            return Some(temp_char);
        };
        
        if self.debug { println!("going to consuem next char") };
        self.it.next()
    }

    #[inline]
    fn set_temp(&mut self, temp_char: char) {
        self.temp_char = Some(temp_char);
        if self.debug {
            println!("set temp: {:?}", temp_char)
        }
    }

    fn parse_ident(&mut self) {
        let mut len = 1;
        while let Some(char) = self.next() {
            match char {
                'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => len += 1,
                '.' => {
                    self.push(TokenKind::Ident, len);
                    self.push(TokenKind::Dot, 1);
                    return;
                }
                _ => {
                    self.push(TokenKind::Ident, len);
                    self.set_temp(char);
                    return;
                }
            };
        }
    }

    fn parse_number(&mut self, found_dot: bool) {
        let mut len = if found_dot { 2 } else { 1 };
        while let Some(char) = self.next() {
            match char {
                '0'..='9' | '.' | '_' | 'a'..='z' | 'A'..='Z' => len += 1,
                _ => {
                    self.push(TokenKind::Number, len);
                    self.set_temp(char);
                    return;
                }
            };
        }
    }

    fn parse_whitespace(&mut self) {
        let mut len = 1;

        while let Some(char) = self.next() {
            if char != ' ' {
                self.push(TokenKind::WhiteSpace, len);
                self.set_temp(char);
                return;
            };
            len += 1;
        }
    }

    fn parse_char(&mut self, first: char) {
        match first {
            ' ' => self.parse_whitespace(),
            '\n' => self.push(TokenKind::Newline, 1),
            '+' => self.push(TokenKind::Plus, 1),
            '-' => self.push(TokenKind::Minus, 1),
            '*' => self.push(TokenKind::Star, 1),
            '/' => self.push(TokenKind::Slash, 1),
            '%' => self.push(TokenKind::Percent, 1),
            '(' => self.push(TokenKind::OpenParen, 1),
            ')' => self.push(TokenKind::CloseParen, 1),
            '.' => 'arm: {
                if let Some(char) = self.next() {
                    if char.is_numeric() {
                        self.parse_number(true);
                        break 'arm;
                    } else {
                        self.push(TokenKind::Dot, 1);
                        self.set_temp(char);
                    };
                }
            }
            '0'..='9' => self.parse_number(false),
            'a'..='z' | 'A'..='Z' | '_' => self.parse_ident(),
            _ => self.push(TokenKind::InvalidChar, first.len_utf8()),
        };
    }

    pub fn tokenize<Output: From<Lexer<'src>>>(mut self, debug: bool) -> Output {
        self.debug = debug;

        while let Some(char) = self.next() {
            self.parse_char(char);
        };

        Output::from(self)
    }
}
