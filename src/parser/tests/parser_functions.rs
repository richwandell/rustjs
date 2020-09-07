use crate::lexer::lexer::Lexer;
use std::fs;
use crate::parser::parser::Parser;
use crate::parser::symbols::{Expression, Statement};
use crate::parser::symbols::JSItem;
use crate::lexer::js_token::Tok;

#[test]
fn test_simple_function_declaration() {
    let file = fs::read_to_string("js/functions/function.js");

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

#[test]
fn test_let_arrow_function_declaration() {
    let file = fs::read_to_string("js/functions/let_arrow.js");

    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(file.unwrap());
    let js_items = parser.parse(tokens);

    assert_eq!(js_items.len(), 1);
    let function = js_items.get(0).unwrap();
    assert!(function.eq(&JSItem::St {
        statement: Box::new(Statement::AssignArrowFunction {
            mutable: true,
            function: Box::new(Statement::FunctionDef {
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
            }),
        })
    }))
}

#[test]
fn test_let_function_declaration() {
    let file = fs::read_to_string("js/functions/let_function.js");

    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(file.unwrap());
    let js_items = parser.parse(tokens);

    assert_eq!(js_items.len(), 1);
    let function = js_items.get(0).unwrap();
    assert!(function.eq(&JSItem::St {
        statement: Box::new(Statement::AssignFunction {
            mutable: true,
            function: Box::new(Statement::FunctionDef {
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
            }),
        })
    }))
}

#[test]
fn test_const_arrow_function_declaration() {
    let file = fs::read_to_string("js/functions/const_arrow.js");

    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(file.unwrap());
    let js_items = parser.parse(tokens);

    assert_eq!(js_items.len(), 1);
    let function = js_items.get(0).unwrap();
    assert!(function.eq(&JSItem::St {
        statement: Box::new(Statement::AssignArrowFunction {
            mutable: false,
            function: Box::new(Statement::FunctionDef {
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
            }),
        })
    }))
}

#[test]
fn test_const_function_declaration() {
    let file = fs::read_to_string("js/functions/const_function.js");

    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(file.unwrap());
    let js_items = parser.parse(tokens);

    assert_eq!(js_items.len(), 1);
    let function = js_items.get(0).unwrap();
    assert!(function.eq(&JSItem::St {
        statement: Box::new(Statement::AssignFunction {
            mutable: false,
            function: Box::new(Statement::FunctionDef {
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
            }),
        })
    }))
}