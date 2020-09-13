use crate::lexer::lexer::Lexer;
use std::fs;
use crate::parser::parser::Parser;
use crate::parser::symbols::{Expression, Statement};
use crate::parser::symbols::JSItem;
use crate::lexer::js_token::Tok;

#[test]
fn test_for() {
    let file = fs::read_to_string("js/if_while_for/for1.js");

    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(file.unwrap());
    let js_items = parser.parse(tokens);

    assert_eq!(js_items.len(), 1);
    let function = js_items.get(0).unwrap();
    assert!(function.eq(&JSItem::St {
        statement: Box::new(Statement::FunctionDef {
            name: "f".to_string(),
            params: vec![],
            body: vec![JSItem::Ex {
                expression: Box::new(Expression::CallExpression {
                    callee: Box::new(Expression::MemberExpression {
                        object: Box::new(Expression::Identifier { name: "console".to_string() }),
                        property: Box::new(Expression::Identifier { name: "log".to_string() }),
                    }),
                    arguments: vec![Tok::String { value: "hi".to_string() }],
                })
            }],
        })
    }))
}