use std::fs;
use crate::lexer::lexer::Lexer;
use crate::parser::parser::Parser;
use std::collections::HashMap;
use crate::parser::symbols::{JSItem, Expression, Statement, AssignOp, Operator};
use crate::lexer::js_token::Tok::Static;

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

#[test]
fn test_edit_object_and_log_from_another_object() {
    let file = fs::read_to_string("js/objects/edit_object_and_log_from_another_object.js");
    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(file.unwrap());
    let js_items = parser.parse(tokens);

    assert_eq!(js_items.len(), 3);

    let statement1 = js_items.get(0).unwrap();
    let mut properties1 = HashMap::new();
    properties1.insert("a".to_string(), JSItem::Ex {
        expression: Box::from(Expression::Number { value: 1.0 })
    });
    assert!(statement1.eq(&JSItem::St {
        statement: Box::new(Statement::AssignmentExpression {
            operator: AssignOp::Let,
            left: JSItem::Ex {
                expression: Box::from(Expression::Literal { value: "b".to_string() })
            },
            right: JSItem::Object {
                mutable: true,
                properties: properties1
            }
        })
    }));

    let statement2 = js_items.get(1).unwrap();
    let mut properties2 = HashMap::new();
    properties2.insert("run".to_string(), JSItem::Ex {
        expression: Box::new(Expression::FuncEx {
            params: vec![],
            body: vec![JSItem::St {
                statement: Box::new(Statement::ForStatement {
                    init: JSItem::St {
                        statement: Box::new(Statement::AssignmentExpression {
                            operator: AssignOp::Let,
                            left: JSItem::Ex {
                                expression: Box::new(Expression::Literal {value: "a".to_string()})
                            },
                            right: JSItem::Ex {
                                expression: Box::new(Expression::Number {value: 0.0})
                            }
                        })
                    },
                    test: JSItem::Ex {
                        expression: Box::new(Expression::Binop {
                            a: Box::new(Expression::Identifier {name: "a".to_string()}),
                            op: Operator::Less,
                            b: Box::new(Expression::Number {value: 10.0})
                        })
                    },
                    update: JSItem::Ex {
                        expression: Box::new(Expression::UpdateExpression {
                            expression: Box::new(Expression::Identifier {name: "a".to_string()})
                        })
                    },
                    body: vec![JSItem::St {
                        statement: Box::new(Statement::AssignmentExpression {
                            operator: AssignOp::None,
                            left: JSItem::Ex {
                                expression: Box::new(Expression::MemberExpression {
                                    object: Box::new(Expression::Identifier {name: "b".to_string()}),
                                    property: Box::new(Expression::Identifier {name: "a".to_string()})
                                })
                            },
                            right: JSItem::Ex {
                                expression: Box::new(Expression::Binop {
                                    a: Box::new(Expression::Identifier {name: "a".to_string()}),
                                    op: Operator::Mult,
                                    b: Box::new(Expression::Number {value: 2.0})
                                })
                            }
                        })
                    }, JSItem::Ex {
                        expression: Box::new(Expression::CallExpression {
                            callee: Box::new(Expression::MemberExpression {
                                object: Box::new(Expression::Identifier {name: "console".to_string()}),
                                property: Box::new(Expression::Identifier {name: "log".to_string()})
                            }),
                            arguments: vec![JSItem::Ex {
                                expression: Box::new(Expression::MemberExpression {
                                    object: Box::new(Expression::Identifier {name: "b".to_string()}),
                                    property: Box::new(Expression::Identifier {name: "a".to_string()})
                                })
                            }]
                        })
                    }]
                })
            }]
        })
    });
    assert!(statement2.eq(&JSItem::St {
        statement: Box::new(Statement::AssignmentExpression {
            operator: AssignOp::Let,
            left: JSItem::Ex {
                expression: Box::new(Expression::Literal {value: "a".to_string()})
            },
            right: JSItem::Object {
                mutable: true,
                properties: properties2
            }
        })
    }))
}