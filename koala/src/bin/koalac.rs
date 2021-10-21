use std::env::args;

use koala::grammar::parser::parse;

fn main() {
    let files: Vec<String> = args().collect();

    if files.len() > 1 {
        let ast = parse(&files[1]);
        
    }
}
