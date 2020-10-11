use std::fs;
use crate::lexer::lexer::Lexer;
use crate::parser::parser::Parser;
use crate::parser::symbols::{JSItem, Expression, Operator, Statement, AssignOp};
use std::collections::HashMap;
use crate::lexer::js_token::Tok;

#[test]
fn test_object_expression_0() {
    let file = fs::read_to_string("js/objects/object_expression_0.js");
    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(file.unwrap());
    let js_items = parser.parse(tokens);

    assert_eq!(js_items.len(), 1);

    let mut properties = HashMap::new();
    properties.insert("a".to_string(), JSItem::Ex {
        expression: Box::new(Expression::Number { value: 1. })
    });
    properties.insert("b".to_string(), JSItem::Ex {
        expression: Box::new(Expression::Number { value: 2. })
    });

    let object = js_items.get(0).unwrap();
    assert!(object.eq(&JSItem::Object {
        mutable: true,
        properties
    }));
}

#[test]
fn test_object_expression_1() {
    let file = fs::read_to_string("js/objects/object_expression_1.js");
    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(file.unwrap());
    let js_items = parser.parse(tokens);

    assert_eq!(js_items.len(), 1);

    let mut properties = HashMap::new();
    properties.insert("a".to_string(), JSItem::Ex {
        expression: Box::new(Expression::Binop {
            a: Box::new(Expression::Number {value: 1.}),
            op: Operator::Add,
            b: Box::new(Expression::Number {value: 2.})
        })
    });
    properties.insert("b".to_string(), JSItem::Ex {
        expression: Box::new(Expression::FuncEx {
            params: vec![
                Tok::Name { name: "a".to_string() },
                Tok::Name { name: "b".to_string() },
                Tok::Name { name: "c".to_string() },
            ],
            body: vec![JSItem::St {
                statement: Box::from(Statement::Return {
                    value: Box::new(JSItem::Ex {
                        expression: Box::new(Expression::Binop {
                            a: Box::new(Expression::Binop {
                                a: Box::new(Expression::Identifier {name: "a".to_string()}),
                                op: Operator::Add,
                                b: Box::new(Expression::Identifier {name: "b".to_string()})
                            }),
                            op: Operator::Add,
                            b: Box::new(Expression::Identifier {name: "c".to_string()})
                        })
                    })
                })
            }]
        })
    });

    let object = js_items.get(0).unwrap();
    assert!(object.eq(&JSItem::Object {
        mutable: true,
        properties
    }));
}

#[test]
fn test_object_object_call_property() {
    let file = fs::read_to_string("js/objects/object_object_call_property.js");
    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(file.unwrap());
    let js_items = parser.parse(tokens);

    assert_eq!(js_items.len(), 2);
    let object = js_items.get(0).unwrap();
    let mut properties1 = HashMap::new();
    let mut properties2 = HashMap::new();
    let mut properties3 = HashMap::new();
    properties3.insert("foo".to_string(), JSItem::Ex {
        expression: Box::new(Expression::Number {value: 1.0})
    });
    properties2.insert("baz".to_string(), JSItem::Ex {
        expression: Box::new(Expression::FuncEx {
            params: vec![],
            body: vec![JSItem::St {
                statement: Box::new(Statement::Return {
                    value: Box::from(JSItem::Object {
                        mutable: true,
                        properties: properties3
                    })
                })
            }]
        })
    });
    properties1.insert("bar".to_string(), JSItem::Object {
        mutable: true,
        properties: properties2
    });
    assert!(object.eq(&JSItem::St {
        statement: Box::new(Statement::AssignmentExpression {
            operator: AssignOp::Let,
            left: JSItem::Ex {
                expression: Box::new(Expression::Literal {value: "foo".to_string()})
            },
            right: JSItem::Object {
                mutable: true,
                properties: properties1
            }
        })
    }));

    let expression = js_items.get(1).unwrap();
    assert!(expression.eq(&JSItem::Ex {
        expression: Box::new(Expression::MemberExpression {
            object: Box::new(Expression::CallExpression {
                callee: Box::new(Expression::MemberExpression {
                    object: Box::new(Expression::MemberExpression {
                        object: Box::new(Expression::Identifier {name: "foo".to_string()}),
                        property: Box::new(Expression::Identifier {name: "bar".to_string()})
                    }),
                    property: Box::new(Expression::Identifier {name: "baz".to_string()})
                }),
                arguments: vec![]
            }),
            property: Box::new(Expression::Identifier {name: "foo".to_string()})
        })
    }))
}
