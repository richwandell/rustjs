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
        vec![JSItem::Variable{ mutable: true, value: Expression::Number {value: 0.}}],
        vec![JSItem::Variable{ mutable: true, value: Expression::Number {value: 1.}}],
        vec![JSItem::Variable{ mutable: true, value: Expression::Number {value: 2.}}],
        vec![JSItem::Variable{ mutable: true, value: Expression::Number {value: 3.}}],
        vec![JSItem::Variable{ mutable: true, value: Expression::Number {value: 4.}}],
        vec![JSItem::Variable{ mutable: true, value: Expression::Number {value: 5.}}],
        vec![JSItem::Variable{ mutable: true, value: Expression::Number {value: 6.}}],
        vec![JSItem::Variable{ mutable: true, value: Expression::Number {value: 7.}}],
        vec![JSItem::Variable{ mutable: true, value: Expression::Number {value: 8.}}],
        vec![JSItem::Variable{ mutable: true, value: Expression::Number {value: 9.}}],
    ]))
}