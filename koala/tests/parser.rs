use std::fs;

use crate::utils::fixture_path;
use koala::grammar::parser::parse;

mod utils;

#[test]
fn parse_program1() {
    let code = match fs::read_to_string(fixture_path("program1.json")) {
        Ok(file_content) => file_content,
        Err(_) => String::from(""),
    };

    let program = parse(&code);

    let json = match serde_json::to_string_pretty(&program) {
        Ok(s) => s,
        Err(_) => return,
    };

    println!("{}", json);
}

#[test]
fn parse_program2() {
    let code = match fs::read_to_string(fixture_path("program2.json")) {
        Ok(file_content) => file_content,
        Err(_) => String::from(""),
    };

    let program = parse(&code);

    let json = match serde_json::to_string_pretty(&program) {
        Ok(s) => s,
        Err(_) => return,
    };

    println!("{}", json);
}
