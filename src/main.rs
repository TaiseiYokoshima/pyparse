mod errors;


mod lex;
use lex::{Lexer};

mod parser;
use parser::Parser;

mod start;
mod units;

fn main() {
    let src = {
        let path = start::parse_path();
        start::load_src(&path)
    };

    let mut lexer = Lexer::new(&src);
    let result = lexer.tokenize();

    match result {
        Ok(_) => println!("\n\n\n{}", lexer),
        Err(e) => println!("{:?}", e),

    };

    // println!("\n\n\n{:?}", result);

    // let parser = Parser::new(lexer);
    // let expr = parser.parse();
    // println!("\n\n\nexpr:\n{}", expr);
}
