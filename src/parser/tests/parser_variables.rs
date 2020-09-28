use crate::lexer::lexer::Lexer;
use std::fs;
use crate::parser::parser::Parser;
use crate::parser::symbols::{Expression, Statement, Operator, AssignOp};
use crate::parser::symbols::JSItem;
use crate::lexer::js_token::Tok;

#[test]
fn test_let_variable_declaration() {
    let file = fs::read_to_string("js/variables/let_a.js");

    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(file.unwrap());
    let js_items = parser.parse(tokens);

    assert_eq!(js_items.len(), 1);
    let function = js_items.get(0).unwrap();
    assert!(function.eq(&JSItem::St {
        statement: Box::new(Statement::AssignmentExpression {
            operator: AssignOp::Let,
            left: JSItem::Ex {expression: Box::from(Expression::Literal {value: "a".to_string()})},
            right: JSItem::Ex {expression: Box::from(Expression::Literal { value: "hi".to_string() })}
        })
    }))
}

#[test]
fn test_let_variable_declaration_number() {
    let file = fs::read_to_string("js/variables/let_a_number.js");

    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(file.unwrap());
    let js_items = parser.parse(tokens);

    assert_eq!(js_items.len(), 1);
    let function = js_items.get(0).unwrap();
    assert!(function.eq(&JSItem::St {
        statement: Box::new(Statement::AssignmentExpression {
            operator: AssignOp::Let,
            left: JSItem::Ex {expression: Box::from(Expression::Literal {value: "a".to_string()})},
            right: JSItem::Ex {expression: Box::from(Expression::Number { value: 123. })}
        })
    }))
}

#[test]
fn test_let_math_declaration() {
    let file = fs::read_to_string("js/variables/let_a_math.js");

    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(file.unwrap());
    let js_items = parser.parse(tokens);

    assert_eq!(js_items.len(), 1);
    let function = js_items.get(0).unwrap();
    assert!(function.eq(&JSItem::St {
        statement: Box::new(Statement::AssignmentExpression {
            operator: AssignOp::Let,
            left: JSItem::Ex {expression: Box::from(Expression::Literal {value: "a".to_string()})},
            right: JSItem::Ex {expression: Box::new(Expression::SubExpression {
                expression: Box::new(Expression::Binop {
                    a: Box::new(Expression::Number { value: 1. }),
                    op: Operator::Add,
                    b: Box::new(Expression::Number { value: 2. }),
                })
            })}
        })
    }))
}

#[test]
fn test_let_math_declaration1() {
    let file = fs::read_to_string("js/variables/let_a_math1.js");

    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(file.unwrap());
    let js_items = parser.parse(tokens);

    assert_eq!(js_items.len(), 1);
    let function = js_items.get(0).unwrap();
    assert!(function.eq(&JSItem::St {
        statement: Box::new(Statement::AssignmentExpression {
            operator: AssignOp::Let,
            left: JSItem::Ex {expression: Box::from(Expression::Literal {value: "a".to_string()})},
            right: JSItem::Ex {expression: Box::new(Expression::Binop {
                a: Box::new(Expression::Number { value: 1. }),
                op: Operator::Add,
                b: Box::new(Expression::Binop {
                    a: Box::new(Expression::Number {value: 2.}),
                    op: Operator::Mult,
                    b: Box::new(Expression::SubExpression {
                        expression: Box::new(Expression::Binop {
                            a: Box::new(Expression::Number {value: 3.}),
                            op: Operator::Add,
                            b: Box::new(Expression::Number {value: 2.})
                        })
                    })
                }),
            })}
        })
    }))
}

#[test]
fn test_let_var_plus_var() {
    let file = fs::read_to_string("js/variables/let_a_math1.js");

    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(file.unwrap());
    let js_items = parser.parse(tokens);

    assert_eq!(js_items.len(), 1);
    let function = js_items.get(0).unwrap();
    assert!(function.eq(&JSItem::St {
        statement: Box::new(Statement::AssignmentExpression {
            operator: AssignOp::Let,
            left: JSItem::Ex {expression: Box::from(Expression::Literal {value: "a".to_string()})},
            right: JSItem::Ex {expression: Box::new(Expression::Binop {
                a: Box::new(Expression::Number { value: 1. }),
                op: Operator::Add,
                b: Box::new(Expression::Binop {
                    a: Box::new(Expression::Number {value: 2.}),
                    op: Operator::Mult,
                    b: Box::new(Expression::SubExpression {
                        expression: Box::new(Expression::Binop {
                            a: Box::new(Expression::Number {value: 3.}),
                            op: Operator::Add,
                            b: Box::new(Expression::Number {value: 2.})
                        })
                    })
                }),
            })}
        })
    }))
}