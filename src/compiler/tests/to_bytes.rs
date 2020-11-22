use crate::lexer::lexer::Lexer;
use crate::parser::parser::Parser;
use crate::compiler::compiler::{Compiler};
use crate::compiler::to_bytes::{to_bytes};
use crate::compiler::op_codes::Op;

#[test]
fn test_math_1() {
    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(String::from("1 + 2 + 3"));
    let mut js_items = parser.parse(tokens);

    let mut com = Compiler::new();
    let item = js_items.pop().unwrap();
    com.compile(item);

    let bytes = to_bytes(com.bc_ins);

    assert_eq!(bytes, vec![
        7, 63, 240, 0, 0, 0, 0, 0, 0, 7, 64, 0, 0, 0, 0, 0, 0, 0, 2, 7, 64, 8, 0, 0, 0, 0, 0, 0, 2
    ])
}

#[test]
fn test_console_log() {
    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(String::from("console.log(\"hi\");"));
    let mut js_items = parser.parse(tokens);

    let mut com = Compiler::new();
    let item = js_items.pop().unwrap();
    com.compile(item);

    let bytes = to_bytes(com.bc_ins);

    assert_eq!(bytes, vec![10, 0, 0, 0, 7, 99, 111, 110, 115, 111, 108, 101, 19, 0, 0, 0, 3, 108,
                           111, 103, 8, 0, 0, 0, 2, 104, 105, 12, 1]);
}

