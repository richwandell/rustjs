use crate::lexer::lexer::Lexer;
use std::fs;
use crate::ast::parser::Parser;

#[test]
fn test_parser() {
    let file = fs::read_to_string("js/math/div/div4.js");

    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let expressions = lex.parse(parser, file.unwrap());

    println!("hi")
}