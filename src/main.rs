mod units;
mod start;
// mod errors;

// mod lex;
// mod parser;


pub use units::Cursor;

fn main() {
    let src = {
        let path = start::parse_path();
        start::load_src(&path)
    };

    
    let debug = true;
    // let debug = false;
    let cursor = Cursor::new(&src);
    let tokens = cursor.tokenize(debug);


    println!("\n\n");
    println!("{}", tokens);

}
