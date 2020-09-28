use crate::parser::parser::Parser;
use std::fs;
use crate::lexer::lexer::Lexer;
use crate::vm::interpreter::Interpreter;
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
    let mut out = JSItem::Null;
    for item in js_items {
        out = int.interpret(item);
    }
    assert_eq!(out, JSItem::Null);
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