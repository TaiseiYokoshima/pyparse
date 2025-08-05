mod tokenizer;
use tokenizer::{Lexer, Tokenizer};

mod parser;
use parser::Parser;

mod start;
mod units;

fn main() {
    let src = {
        let path = start::parse_path();
        start::load_src(&path)
    };

    let tokenizer = Tokenizer::new(src);
    let lexer = tokenizer.tokenize();

    println!("\n\n\n{}", lexer);

    let parser = Parser::new(lexer);
    let expr = parser.parse();

    println!("\n\n\nexpr:\n{}", expr);
}
