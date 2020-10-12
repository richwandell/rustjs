use crate::parser::parser::Parser;
use std::fs;
use crate::lexer::lexer::Lexer;
use crate::vm::interpreter::Interpreter;
use crate::parser::symbols::JSItem;
use std::collections::HashMap;

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
        properties: hashmap!{
            "length".to_string() => JSItem::Number {value: 5. }
        }
    });
}

#[test]
fn test_array_push() {
    let file = fs::read_to_string("js/arrays/push.js");

    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(file.unwrap());
    let mut js_items = parser.parse(tokens);

    let mut int = Interpreter::new();
    let mut out = JSItem::Undefined;
    for item in js_items {
        out = int.interpret(item);
    }

    assert!(out.eq(&JSItem::Undefined));
    assert_eq!(int.captured_output.len(), 1);

    let mut out = int.captured_output;
    let mut item = out.pop().unwrap();
    assert_eq!(item.len(), 1);

    if let JSItem::Array { items, properties:_ } = item.pop().unwrap() {
        assert!(items.eq(&vec![JSItem::Number {value: 1.}]))
    }
}