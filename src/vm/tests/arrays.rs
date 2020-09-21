use crate::parser::parser::Parser;
use std::fs;
use crate::lexer::lexer::Lexer;
use crate::vm::interpreter::Interpreter;
use crate::parser::symbols::JSItem;

#[test]
fn test_array_apply() {
    let file = fs::read_to_string("js/arrays/array_apply.js");

    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(file.unwrap());
    let mut js_items = parser.parse(tokens);

    let mut int = Interpreter::new();
    let out = int.interpret(js_items.pop().unwrap());
    assert_eq!(out, JSItem::Array {
        items: vec![
            JSItem::Undefined,
            JSItem::Undefined,
            JSItem::Undefined,
            JSItem::Undefined,
            JSItem::Undefined
        ],
        length: 5
    });
}