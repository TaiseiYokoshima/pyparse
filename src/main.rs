mod tokenizer;
use tokenizer::Tokenizer;

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
    let tokens = tokenizer.tokenize();

    println!("\n\n\ntokens:\n{:?}", tokens);

    let parser = Parser::new(tokens);
    let expr = parser.parse();

    println!("\n\n\nexpr:\n{}", expr);
}
