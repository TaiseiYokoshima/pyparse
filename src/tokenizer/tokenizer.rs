use super::Token;

#[derive(Debug)]
pub struct Tokenizer {
    tokens: Vec<Token>,
    builder: Box<String>,
    src: String,
}

impl Tokenizer {
    pub fn new(src: String) -> Self {
        Tokenizer {
            src,
            tokens: vec![],
            builder: Box::default(),
        }
    }

    pub fn tokenize(mut self) -> Vec<Token> {
        while let Some(char) = self.src.chars().next() {
            self.src.drain(..char.len_utf8());
            self.parse_char(char);
        }

        self.parse_token();
        return self.tokens;
    }

    fn parse_char(&mut self, char: char) {
        if let Some(deliminiter) = Token::match_deliminiter(&char) {
            if !self.builder.is_empty() {
                self.parse_token();
            };

            if deliminiter != Token::Space {
                self.add_token(deliminiter);
            }

            return;
        };

        self.builder.push(char);
    }

    fn add_token(&mut self, token: Token) {
        println!("parsed: {:?}", token);
        self.tokens.push(token);
    }

    fn parse_token(&mut self) {
        if self.builder.is_empty() {
            return;
        }

        let token = Token::match_token(&mut self.builder);
        self.add_token(token);
    }
}
