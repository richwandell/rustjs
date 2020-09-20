use crate::lexer::lexer::Lexer;
use std::fs;
use crate::parser::parser::Parser;
use crate::parser::symbols::{Expression, Statement, Operator, AssignOp};
use crate::parser::symbols::JSItem;
use crate::lexer::js_token::Tok;

#[test]
fn test_number_array() {
    let file = fs::read_to_string("js/arrays/number_array.js");
    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(file.unwrap());
    let js_items = parser.parse(tokens);

    assert_eq!(js_items.len(), 1);
    let function = js_items.get(0).unwrap();
    assert!(function.eq(&JSItem::St {
        statement: Box::new(Statement::AssignExpression {
            assign_op: AssignOp::Let,
            name: "a".to_string(),
            value: Box::new(Expression::ArrayExpression {
                items: vec![
                    JSItem::Ex { expression: Box::new(Expression::Number {value: 1.}) },
                    JSItem::Ex { expression: Box::new(Expression::Number {value: 2.}) },
                    JSItem::Ex { expression: Box::new(Expression::Number {value: 3.}) },
                    JSItem::Ex { expression: Box::new(Expression::Number {value: 4.}) },
                    JSItem::Ex { expression: Box::new(Expression::Number {value: 5.}) },
                    JSItem::Ex { expression: Box::new(Expression::Number {value: 6.}) },
                    JSItem::Ex { expression: Box::new(Expression::Number {value: 7.}) },
                    JSItem::Ex { expression: Box::new(Expression::Number {value: 8.}) },
                    JSItem::Ex { expression: Box::new(Expression::Number {value: 9.}) },
                    JSItem::Ex { expression: Box::new(Expression::Number {value: 10.}) },
                    JSItem::Ex { expression: Box::new(Expression::Number {value: 11.}) }
                ]
            })
        })
    }))
}

#[test]
fn test_array_object_apply() {
    let file = fs::read_to_string("js/arrays/array_object_apply.js");
    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(file.unwrap());
    let js_items = parser.parse(tokens);

    assert_eq!(js_items.len(), 1);
}