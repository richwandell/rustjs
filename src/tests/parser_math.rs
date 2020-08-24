use crate::lexer::lexer::Lexer;
use std::fs;
use crate::ast::parser::Parser;

#[test]
fn test_parser() {
    let file = fs::read_to_string("js/function_expression.js");

    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(file.unwrap());
    let expressions = parser.parse(tokens);

    println!("hi")
}