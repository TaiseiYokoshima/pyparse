use std::collections::VecDeque;
use std::fmt;
use std::str::Chars;

pub struct Cursor<'src> {
    it: Chars<'src>,
    tokens: VecDeque<Token>,
    temp_char: Option<char>,
    debug: bool,
}

impl<'src> Cursor<'src> {
    pub fn new(src: &'src str) -> Cursor<'src> {
        let it = src.chars();
        let tokens = VecDeque::default();
        let temp_char = None;

        Self {
            it, 
            tokens,
            temp_char,
            debug: false,
        }
    }

    #[inline]
    fn push(&mut self, kind: TokenKind, len: usize) {
        let token = Token::new(kind, len);
        if self.debug { println!("Token: {}", &token) };
        self.tokens.push_back(token);
    }

    #[inline]
    fn next(&mut self) -> Option<char> {
        self.it.next()
    }

    #[inline]
    fn set_temp(&mut self, temp_char: char) {
        self.temp_char = Some(temp_char);
        if self.debug { println!("set temp: {:?}", temp_char) }
    }

    #[inline]
    fn get_temp(&mut self) -> Option<char> {
        let char = self.temp_char?;
        self.temp_char = None;
        if self.debug { println!("got temp: {:?}", char) };
        Some(char)
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
                println!("got non space: {}", char);
                self.set_temp(char);
                return;
            };
            len += 1;
        };
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
            },
            '0'..='9' => self.parse_number(false),
            'a'..='z' | 'A'..='Z' | '_' => self.parse_ident(),
            _ => self.push(TokenKind::InvalidChar, first.len_utf8()),
        };
    }

    pub fn tokenize(mut self, debug: bool) -> Tokens {
        self.debug = debug;

        loop {
            if let Some(temp_char) = self.get_temp() {
                self.temp_char = None;
                self.parse_char(temp_char);
                continue;
            };

            if let Some(char) = self.next() {
                self.parse_char(char);
            } else {
                break;
            };
        };

        Tokens(self.tokens)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    kind: TokenKind,
    len: usize,
}

impl Token {
    fn new(kind: TokenKind, len: usize) -> Self {
        Self { kind, len }
    }
}


impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let size = self.len;

        match self.kind {
            TokenKind::Dot => write!(f, "'.' : {}", size),
            TokenKind::Plus => write!(f, "'+' : {}", size),
            TokenKind::Minus => write!(f, "'-' : {}", size),
            TokenKind::Star => write!(f, "'*' : {}", size),
            TokenKind::Slash => write!(f, "'/' : {}", size),
            TokenKind::Percent => write!(f, "'%' : {}", size),
            TokenKind::OpenParen => write!(f, "'(' : {}", size),
            TokenKind::CloseParen => write!(f, "')' : {}", size),
            TokenKind::Number => write!(f, "Number : {}", size),
            TokenKind::Newline => write!(f, "Newline : {}", size),
            TokenKind::Ident => write!(f, "Ident : {}", size),
            TokenKind::WhiteSpace => write!(f, "WhiteSpace : {}", size),
            TokenKind::InvalidChar => write!(f, "Invalid Char : {}", size),
            TokenKind::Semi => write!(f, "Semi : {}", size),
            TokenKind::Colon => write!(f, "Colon : {}", size),
            TokenKind::Comma => write!(f, "Comma : {}", size),
        }
    }
}




pub struct Tokens(VecDeque<Token>);


impl fmt::Display for Tokens {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tokens [\n")?;
        for token in &self.0 {
            write!(f, "{}\n", token)?;
        };

        write!(f, "]")
    }

}


#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind {
    WhiteSpace,
    Newline,

    OpenParen,
    CloseParen,

    Plus,
    Minus,
    Slash,
    Star,
    Percent,

    Dot,
    Comma,
    Semi,
    Colon,

    Ident,
    Number,
    InvalidChar,
}
