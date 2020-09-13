use crate::lexer::lexer::Lexer;
use crate::parser::parser::Parser;
use crate::vm::interpreter::Interpreter;
use crate::vm::js_output::JSOutput;
use std::fs;
use crate::parser::symbols::{JSItem, Expression};

#[test]
fn test_simple_for() {
    let file = fs::read_to_string("js/if_while_for/for1.js");

    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(file.unwrap());
    let mut js_items = parser.parse(tokens);

    let mut int = Interpreter::new();
    let out = int.interpret(js_items.pop().unwrap());
    assert_eq!(out, JSOutput::Null);
    let captured = int.captured_output;
    assert_eq!(captured.len(), 10);
    assert!(captured.eq(&vec![
        vec![JSItem::Number {value: 0.}],
        vec![JSItem::Number {value: 1.}],
        vec![JSItem::Number {value: 2.}],
        vec![JSItem::Number {value: 3.}],
        vec![JSItem::Number {value: 4.}],
        vec![JSItem::Number {value: 5.}],
        vec![JSItem::Number {value: 6.}],
        vec![JSItem::Number {value: 7.}],
        vec![JSItem::Number {value: 8.}],
        vec![JSItem::Number {value: 9.}],
    ]))
}

#[test]
fn test_nested_for() {
    let file = fs::read_to_string("js/if_while_for/nested_for.js");

    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(file.unwrap());
    let mut js_items = parser.parse(tokens);

    let mut int = Interpreter::new();
    let out = int.interpret(js_items.pop().unwrap());
    assert_eq!(out, JSOutput::Null);
    let captured = int.captured_output;
    assert_eq!(captured.len(), 25);
    assert!(captured.eq(&vec![
        vec![JSItem::Number {value: 0.}, JSItem::Number {value: 0.}],
        vec![JSItem::Number {value: 0.}, JSItem::Number {value: 1.}],
        vec![JSItem::Number {value: 0.}, JSItem::Number {value: 2.}],
        vec![JSItem::Number {value: 0.}, JSItem::Number {value: 3.}],
        vec![JSItem::Number {value: 0.}, JSItem::Number {value: 4.}],
        vec![JSItem::Number {value: 1.}, JSItem::Number {value: 0.}],
        vec![JSItem::Number {value: 1.}, JSItem::Number {value: 1.}],
        vec![JSItem::Number {value: 1.}, JSItem::Number {value: 2.}],
        vec![JSItem::Number {value: 1.}, JSItem::Number {value: 3.}],
        vec![JSItem::Number {value: 1.}, JSItem::Number {value: 4.}],
        vec![JSItem::Number {value: 2.}, JSItem::Number {value: 0.}],
        vec![JSItem::Number {value: 2.}, JSItem::Number {value: 1.}],
        vec![JSItem::Number {value: 2.}, JSItem::Number {value: 2.}],
        vec![JSItem::Number {value: 2.}, JSItem::Number {value: 3.}],
        vec![JSItem::Number {value: 2.}, JSItem::Number {value: 4.}],
        vec![JSItem::Number {value: 3.}, JSItem::Number {value: 0.}],
        vec![JSItem::Number {value: 3.}, JSItem::Number {value: 1.}],
        vec![JSItem::Number {value: 3.}, JSItem::Number {value: 2.}],
        vec![JSItem::Number {value: 3.}, JSItem::Number {value: 3.}],
        vec![JSItem::Number {value: 3.}, JSItem::Number {value: 4.}],
        vec![JSItem::Number {value: 4.}, JSItem::Number {value: 0.}],
        vec![JSItem::Number {value: 4.}, JSItem::Number {value: 1.}],
        vec![JSItem::Number {value: 4.}, JSItem::Number {value: 2.}],
        vec![JSItem::Number {value: 4.}, JSItem::Number {value: 3.}],
        vec![JSItem::Number {value: 4.}, JSItem::Number {value: 4.}],
    ]))
}

#[test]
fn test_function_nested_for() {
    let file = fs::read_to_string("js/functions/function_with_loop.js");

    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(file.unwrap());
    let mut js_items = parser.parse(tokens);

    let mut int = Interpreter::new();
    let mut out = JSOutput::Null;
    for item in js_items {
        out = int.interpret(item);
    }

    assert_eq!(out, JSOutput::Null);
    let captured = int.captured_output;
    assert_eq!(captured.len(), 25);
    assert!(captured.eq(&vec![
        vec![JSItem::Number {value: 0.}, JSItem::Number {value: 0.}],
        vec![JSItem::Number {value: 0.}, JSItem::Number {value: 1.}],
        vec![JSItem::Number {value: 0.}, JSItem::Number {value: 2.}],
        vec![JSItem::Number {value: 0.}, JSItem::Number {value: 3.}],
        vec![JSItem::Number {value: 0.}, JSItem::Number {value: 4.}],
        vec![JSItem::Number {value: 1.}, JSItem::Number {value: 0.}],
        vec![JSItem::Number {value: 1.}, JSItem::Number {value: 1.}],
        vec![JSItem::Number {value: 1.}, JSItem::Number {value: 2.}],
        vec![JSItem::Number {value: 1.}, JSItem::Number {value: 3.}],
        vec![JSItem::Number {value: 1.}, JSItem::Number {value: 4.}],
        vec![JSItem::Number {value: 2.}, JSItem::Number {value: 0.}],
        vec![JSItem::Number {value: 2.}, JSItem::Number {value: 1.}],
        vec![JSItem::Number {value: 2.}, JSItem::Number {value: 2.}],
        vec![JSItem::Number {value: 2.}, JSItem::Number {value: 3.}],
        vec![JSItem::Number {value: 2.}, JSItem::Number {value: 4.}],
        vec![JSItem::Number {value: 3.}, JSItem::Number {value: 0.}],
        vec![JSItem::Number {value: 3.}, JSItem::Number {value: 1.}],
        vec![JSItem::Number {value: 3.}, JSItem::Number {value: 2.}],
        vec![JSItem::Number {value: 3.}, JSItem::Number {value: 3.}],
        vec![JSItem::Number {value: 3.}, JSItem::Number {value: 4.}],
        vec![JSItem::Number {value: 4.}, JSItem::Number {value: 0.}],
        vec![JSItem::Number {value: 4.}, JSItem::Number {value: 1.}],
        vec![JSItem::Number {value: 4.}, JSItem::Number {value: 2.}],
        vec![JSItem::Number {value: 4.}, JSItem::Number {value: 3.}],
        vec![JSItem::Number {value: 4.}, JSItem::Number {value: 4.}],
    ]))
}