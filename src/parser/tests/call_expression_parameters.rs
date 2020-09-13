use crate::lexer::lexer::Lexer;
use std::fs;
use crate::parser::parser::Parser;
use crate::parser::symbols::{Expression, Statement, Operator, AssignOp};
use crate::parser::symbols::JSItem;
use crate::lexer::js_token::Tok;

#[test]
fn test_exp_params() {
    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex("console.log((1 + 2), (2 + 2), 3)".to_string());
    let js_items = parser.parse(tokens);

    assert_eq!(js_items.len(), 1);
    let function = js_items.get(0).unwrap();
    assert!(function.eq(&JSItem::Ex {
        expression: Box::new(Expression::CallExpression {
            callee: Box::new(Expression::MemberExpression {
                object: Box::new(Expression::Identifier { name: "console".to_string() }),
                property: Box::new(Expression::Identifier { name: "log".to_string() }),
            }),
            arguments: vec![
                JSItem::Ex {
                    expression: Box::new(Expression::SubExpression {
                        expression: Box::new(Expression::Binop {
                            a: Box::new(Expression::Number { value: 1. }),
                            op: Operator::Add,
                            b: Box::new(Expression::Number { value: 2. }),
                        })
                    })
                },
                JSItem::Ex {
                    expression: Box::new(Expression::SubExpression {
                        expression: Box::new(Expression::Binop {
                            a: Box::new(Expression::Number { value: 2. }),
                            op: Operator::Add,
                            b: Box::new(Expression::Number { value: 2. }),
                        })
                    })
                },
                JSItem::Ex {
                    expression: Box::new(Expression::Number { value: 3. })
                }
            ],
        })
    }))
}

#[test]
fn test_var_plus_number() {
    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex("console.log(a + 2)".to_string());
    let js_items = parser.parse(tokens);

    assert_eq!(js_items.len(), 1);
    let function = js_items.get(0).unwrap();
    assert!(function.eq(&JSItem::Ex {
        expression: Box::new(Expression::CallExpression {
            callee: Box::new(Expression::MemberExpression {
                object: Box::new(Expression::Identifier { name: "console".to_string() }),
                property: Box::new(Expression::Identifier { name: "log".to_string() }),
            }),
            arguments: vec![
                JSItem::Ex {
                    expression: Box::new(Expression::Binop {
                        a: Box::new(Expression::Identifier { name: "a".to_string() }),
                        op: Operator::Add,
                        b: Box::new(Expression::Number { value: 2. }),
                    })
                }
            ],
        })
    }))
}