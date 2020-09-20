use crate::lexer::lexer::Lexer;
use std::fs;
use crate::parser::parser::Parser;
use crate::parser::symbols::{Expression, Statement, Operator, AssignOp};
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
        statement: Box::new(Statement::ForStatement {
            init: JSItem::St {
                statement: Box::new(Statement::AssignExpression {
                    assign_op: AssignOp::Let,
                    name: "i".to_string(),
                    value: Box::new(Expression::Number {value: 0.})
                })
            },
            test: JSItem::Ex {
                expression: Box::new(Expression::Binop {
                    a: Box::new(Expression::Identifier {name: "i".to_string()}),
                    op: Operator::Less,
                    b: Box::new(Expression::Number {value: 10.})
                })
            },
            update: JSItem::Ex {
                expression: Box::new(Expression::UpdateExpression {
                    expression: Box::new(Expression::Identifier {name: "i".to_string()})
                })
            },
            body: vec![JSItem::Ex {
                expression: Box::new(Expression::CallExpression {
                    callee: Box::new(Expression::MemberExpression {
                        object: Box::new(Expression::Identifier { name: "console".to_string() }),
                        property: Box::new(Expression::Identifier {name: "log".to_string()})
                    }),
                    arguments: vec![JSItem::Ex {
                        expression: Box::new(Expression::Identifier {name: "i".to_string()})
                    }]
                })
            }]
        })
    }))
}

#[test]
fn test_for_test_ident() {
    let file = fs::read_to_string("js/if_while_for/for_test_ident.js");

    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(file.unwrap());
    let js_items = parser.parse(tokens);

    assert_eq!(js_items.len(), 1);
    let function = js_items.get(0).unwrap();
    assert!(function.eq(&JSItem::St {
        statement: Box::new(Statement::ForStatement {
            init: JSItem::St {
                statement: Box::new(Statement::AssignExpression {
                    assign_op: AssignOp::Let,
                    name: "i".to_string(),
                    value: Box::new(Expression::Number {value: 0.})
                })
            },
            test: JSItem::Ex {
                expression: Box::new(Expression::Binop {
                    a: Box::new(Expression::Identifier {name: "i".to_string()}),
                    op: Operator::Less,
                    b: Box::new(Expression::Identifier {name: "j".to_string()})
                })
            },
            update: JSItem::Ex {
                expression: Box::new(Expression::UpdateExpression {
                    expression: Box::new(Expression::Identifier {name: "i".to_string()})
                })
            },
            body: vec![JSItem::Ex {
                expression: Box::new(Expression::CallExpression {
                    callee: Box::new(Expression::MemberExpression {
                        object: Box::new(Expression::Identifier { name: "console".to_string() }),
                        property: Box::new(Expression::Identifier {name: "log".to_string()})
                    }),
                    arguments: vec![JSItem::Ex {
                        expression: Box::new(Expression::Identifier {name: "i".to_string()})
                    }]
                })
            }]
        })
    }))
}

#[test]
fn test_for_test_ident_plus_ident() {
    let file = fs::read_to_string("js/if_while_for/for_test_ident_plus_ident.js");

    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(file.unwrap());
    let js_items = parser.parse(tokens);

    assert_eq!(js_items.len(), 1);
    let function = js_items.get(0).unwrap();
    assert!(function.eq(&JSItem::St {
        statement: Box::new(Statement::ForStatement {
            init: JSItem::St {
                statement: Box::new(Statement::AssignExpression {
                    assign_op: AssignOp::Let,
                    name: "j".to_string(),
                    value: Box::new(Expression::Identifier {name: "a".to_string()})
                })
            },
            test: JSItem::Ex {
                expression: Box::new(Expression::Binop {
                    a: Box::new(Expression::Identifier {name: "j".to_string()}),
                    op: Operator::Less,
                    b: Box::new(Expression::Binop {
                        a: Box::new(Expression::Identifier { name: "cols".to_string() }),
                        op: Operator::Add,
                        b: Box::new(Expression::Identifier {name: "a".to_string()})
                    })
                })
            },
            update: JSItem::Ex {
                expression: Box::new(Expression::UpdateExpression {
                    expression: Box::new(Expression::Identifier {name: "j".to_string()})
                })
            },
            body: vec![JSItem::Ex {
                expression: Box::new(Expression::CallExpression {
                    callee: Box::new(Expression::MemberExpression {
                        object: Box::new(Expression::Identifier { name: "console".to_string() }),
                        property: Box::new(Expression::Identifier {name: "log".to_string()})
                    }),
                    arguments: vec![
                        JSItem::Ex {
                            expression: Box::new(Expression::Identifier {name: "i".to_string()})
                        },
                        JSItem::Ex {
                            expression: Box::new(Expression::Identifier {name: "j".to_string()})
                        }
                    ]
                })
            }]
        })
    }))
}