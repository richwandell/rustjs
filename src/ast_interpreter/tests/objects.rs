use crate::parser::parser::Parser;
use std::fs;
use crate::lexer::lexer::Lexer;
use crate::ast_interpreter::interpreter::Interpreter;
use crate::parser::symbols::{JSItem, Expression};
use std::collections::HashMap;

#[test]
fn test_object_new_property() {
    let file = fs::read_to_string("js/objects/object_new_property.js");

    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(file.unwrap());
    let mut js_items = parser.parse(tokens);

    let mut int = Interpreter::new();
    let mut out = JSItem::Undefined;
    for item in js_items {
        out = int.interpret(item);
    }
    assert_eq!(out, JSItem::Undefined);
    let captured = int.captured_output;
    assert_eq!(captured.len(), 1);

    let mut properties = HashMap::new();
    properties.insert("a".to_string(), JSItem::Ex {
        expression: Box::from(Expression::Number {value: 1.0})
    });
    properties.insert("d".to_string(), JSItem::Variable {
        mutable: false,
        value: Expression::String {value: "hello world".to_string()}
    });
    properties.insert("b".to_string(), JSItem::Ex {
        expression: Box::from(Expression::Number {value: 2.0})
    });

    assert!(captured.eq(&vec![
        vec![JSItem::Object {mutable: true,  properties }]
    ]));
}

#[test]
fn test_object_object_call_property() {
    let file = fs::read_to_string("js/objects/object_object_call_property.js");
    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(file.unwrap());
    let js_items = parser.parse(tokens);

    let mut int = Interpreter::new();
    let mut out = JSItem::Null;
    for item in js_items {
        out = int.interpret(item);
    }

    assert!(out.eq(&JSItem::Number {value: 1.0}))
}

#[test]
fn test_log_object_object_call_property() {
    let file = fs::read_to_string("js/objects/log_object_object_call_property.js");
    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(file.unwrap());
    let js_items = parser.parse(tokens);

    let mut int = Interpreter::new();
    let mut out = JSItem::Undefined;
    for item in js_items {
        out = int.interpret(item);
    }
    assert_eq!(out, JSItem::Undefined);
    let captured = int.captured_output;
    assert_eq!(captured.len(), 1);

    assert!(captured.eq(&vec![
        vec![JSItem::Number {value: 1.0}]
    ]))
}

#[test]
fn test_same_variable_different_scope() {
    let file = fs::read_to_string("js/objects/same_variable_different_scope.js");
    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(file.unwrap());
    let js_items = parser.parse(tokens);

    assert_eq!(js_items.len(), 2);

    let mut int = Interpreter::new();
    let mut out = JSItem::Undefined;
    for item in js_items {
        out = int.interpret(item);
    }
    assert_eq!(out, JSItem::Undefined);

    let captured = int.captured_output;
    assert_eq!(captured.len(), 10);

    assert!(captured.eq(&vec![
        vec![JSItem::Number {value: 0.0}],
        vec![JSItem::Number {value: 1.0}],
        vec![JSItem::Number {value: 2.0}],
        vec![JSItem::Number {value: 3.0}],
        vec![JSItem::Number {value: 4.0}],
        vec![JSItem::Number {value: 5.0}],
        vec![JSItem::Number {value: 6.0}],
        vec![JSItem::Number {value: 7.0}],
        vec![JSItem::Number {value: 8.0}],
        vec![JSItem::Number {value: 9.0}]
    ]))
}

#[test]
fn test_edit_object_and_log_from_another_object() {
    let file = fs::read_to_string("js/objects/edit_object_and_log_from_another_object.js");
    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(file.unwrap());
    let js_items = parser.parse(tokens);

    assert_eq!(js_items.len(), 3);

    let mut int = Interpreter::new();
    let mut out = JSItem::Undefined;
    for item in js_items {
        out = int.interpret(item);
    }
    assert_eq!(out, JSItem::Undefined);

    let captured = int.captured_output;
    assert_eq!(captured.len(), 10);

    assert!(captured.eq(&vec![
        vec![JSItem::Number {value: 0.0}],
        vec![JSItem::Number {value: 2.0}],
        vec![JSItem::Number {value: 4.0}],
        vec![JSItem::Number {value: 6.0}],
        vec![JSItem::Number {value: 8.0}],
        vec![JSItem::Number {value: 10.0}],
        vec![JSItem::Number {value: 12.0}],
        vec![JSItem::Number {value: 14.0}],
        vec![JSItem::Number {value: 16.0}],
        vec![JSItem::Number {value: 18.0}]
    ]))
}