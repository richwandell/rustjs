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
                statement: Box::new(Statement::AssignmentExpression {
                    operator: AssignOp::Let,
                    left: JSItem::Ex {expression: Box::from(Expression::Literal {value: "i".to_string()})},
                    right: JSItem::Ex {expression: Box::from(Expression::Number {value: 0.})}
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
                statement: Box::new(Statement::AssignmentExpression {
                    operator: AssignOp::Let,
                    left: JSItem::Ex {expression: Box::from(Expression::Literal {value: "i".to_string()})},
                    right: JSItem::Ex {expression: Box::from(Expression::Number {value: 0.})}
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
                statement: Box::new(Statement::AssignmentExpression {
                    operator: AssignOp::Let,
                    left: JSItem::Ex {expression: Box::from(Expression::Literal {value: "j".to_string()})},
                    right: JSItem::Ex {expression: Box::from(Expression::Identifier {name: "a".to_string()})}
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

#[test]
fn test_for_if() {
    let file = fs::read_to_string("js/if_while_for/for_if.js");

    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(file.unwrap());
    let mut js_items = parser.parse(tokens);

    assert_eq!(js_items.len(), 1);

    let for_loop = js_items.get(0).unwrap();
    assert!(for_loop.eq(&JSItem::St {
        statement: Box::new(Statement::ForStatement {
            init: JSItem::St {
                statement: Box::new(Statement::AssignmentExpression {
                    operator: AssignOp::Let,
                    left: JSItem::Ex {expression: Box::from(Expression::Literal {value: "b".to_string()})},
                    right: JSItem::Ex {expression: Box::from(Expression::Number {value: 0.})}
                })
            },
            test: JSItem::Ex {
                expression: Box::new(Expression::Binop {
                    a: Box::new(Expression::Identifier {name: "b".to_string()}),
                    op: Operator::Less,
                    b: Box::new(Expression::Number {value: 20.})
                })
            },
            update: JSItem::Ex {
                expression: Box::new(Expression::UpdateExpression {
                    expression: Box::new(Expression::Identifier {name: "b".to_string()})
                })
            },
            body: vec![JSItem::St {
                statement: Box::new(Statement::If {
                    test: JSItem::Ex {
                        expression: Box::new(Expression::Binop {
                            a: Box::new(Expression::Identifier {name: "b".to_string()}),
                            op: Operator::Less,
                            b: Box::new(Expression::Number {value: 5.})
                        })
                    },
                    consequent: vec![JSItem::Ex {
                        expression: Box::new(Expression::CallExpression {
                            callee: Box::new(Expression::MemberExpression {
                                object: Box::new(Expression::Identifier {name: "console".to_string()}),
                                property: Box::new(Expression::Identifier {name: "log".to_string()})
                            }),
                            arguments: vec![JSItem::Ex { expression: Box::new(Expression::String {value: "less than 5".to_string()}) }]
                        })
                    }],
                    alternate: JSItem::St {
                        statement: Box::new(Statement::If {
                            test: JSItem::Ex {
                                expression: Box::new(Expression::Binop {
                                    a: Box::new(Expression::Binop {
                                        a: Box::new(Expression::Identifier {name: "b".to_string()}),
                                        op: Operator::Greater,
                                        b: Box::new(Expression::Number {value: 5.})
                                    }),
                                    op: Operator::And,
                                    b: Box::new(Expression::Binop {
                                        a: Box::new(Expression::Identifier {name: "b".to_string()}),
                                        op: Operator::Less,
                                        b: Box::new(Expression::Number {value: 10.})
                                    })
                                })
                            },
                            consequent: vec![JSItem::Ex {
                                expression: Box::new(Expression::CallExpression {
                                    callee: Box::new(Expression::MemberExpression {
                                        object: Box::new(Expression::Identifier {name: "console".to_string()}),
                                        property: Box::new(Expression::Identifier {name: "log".to_string()})
                                    }),
                                    arguments: vec![JSItem::Ex { expression: Box::new(Expression::String {value: "between 5 and 10".to_string()}) }]
                                })
                            }],
                            alternate: JSItem::St { statement: Box::new(Statement::If {
                                test: JSItem::Ex { expression: Box::new(Expression::Binop {
                                    a: Box::new(Expression::Identifier {name: "b".to_string()}),
                                    op: Operator::Greater,
                                    b: Box::new(Expression::Number {value: 10.})
                                }) },
                                consequent: vec![JSItem::Ex {
                                    expression: Box::new(Expression::CallExpression {
                                        callee: Box::new(Expression::MemberExpression {
                                            object: Box::new(Expression::Identifier {name: "console".to_string()}),
                                            property: Box::new(Expression::Identifier {name: "log".to_string()})
                                        }),
                                        arguments: vec![JSItem::Ex { expression: Box::new(Expression::String {value: "greater than 10".to_string()}) }]
                                    })
                                }],
                                alternate: JSItem::Ex {expression: Box::from(Expression::None)}
                            }) }
                        })
                    }
                })
            }]
        })
    }))
}

#[test]
fn test_for_if1() {
    let file = fs::read_to_string("js/if_while_for/for_if1.js");

    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(file.unwrap());
    let mut js_items = parser.parse(tokens);

    assert_eq!(js_items.len(), 1);

    let for_loop = js_items.get(0).unwrap();
    assert!(for_loop.eq(&JSItem::St {
        statement: Box::new(Statement::ForStatement {
            init: JSItem::St {
                statement: Box::new(Statement::AssignmentExpression {
                    operator: AssignOp::Let,
                    left: JSItem::Ex {expression: Box::from(Expression::Literal {value: "b".to_string()})},
                    right: JSItem::Ex {expression: Box::from(Expression::Number {value: 0.})}
                })
            },
            test: JSItem::Ex {
                expression: Box::new(Expression::Binop {
                    a: Box::new(Expression::Identifier {name: "b".to_string()}),
                    op: Operator::Less,
                    b: Box::new(Expression::Number {value: 20.})
                })
            },
            update: JSItem::Ex {
                expression: Box::new(Expression::UpdateExpression {
                    expression: Box::new(Expression::Identifier {name: "b".to_string()})
                })
            },
            body: vec![JSItem::St {
                statement: Box::new(Statement::If {
                    test: JSItem::Ex {
                        expression: Box::new(Expression::Binop {
                            a: Box::new(Expression::Identifier {name: "b".to_string()}),
                            op: Operator::Less,
                            b: Box::new(Expression::Number {value: 5.})
                        })
                    },
                    consequent: vec![JSItem::Ex {
                        expression: Box::new(Expression::CallExpression {
                            callee: Box::new(Expression::MemberExpression {
                                object: Box::new(Expression::Identifier {name: "console".to_string()}),
                                property: Box::new(Expression::Identifier {name: "log".to_string()})
                            }),
                            arguments: vec![JSItem::Ex { expression: Box::new(Expression::String {value: "less than 5".to_string()}) }]
                        })
                    }],
                    alternate: JSItem::St {
                        statement: Box::new(Statement::If {
                            test: JSItem::Ex {
                                expression: Box::new(Expression::Binop {
                                    a: Box::new(Expression::Binop {
                                        a: Box::new(Expression::Number {value: 5.}),
                                        op: Operator::Less,
                                        b: Box::new(Expression::Identifier {name: "b".to_string()})
                                    }),
                                    op: Operator::And,
                                    b: Box::new(Expression::Binop {
                                        a: Box::new(Expression::Identifier {name: "b".to_string()}),
                                        op: Operator::Less,
                                        b: Box::new(Expression::Number {value: 10.})
                                    })
                                })
                            },
                            consequent: vec![JSItem::Ex {
                                expression: Box::new(Expression::CallExpression {
                                    callee: Box::new(Expression::MemberExpression {
                                        object: Box::new(Expression::Identifier {name: "console".to_string()}),
                                        property: Box::new(Expression::Identifier {name: "log".to_string()})
                                    }),
                                    arguments: vec![JSItem::Ex { expression: Box::new(Expression::String {value: "between 5 and 10".to_string()}) }]
                                })
                            }],
                            alternate: JSItem::St { statement: Box::new(Statement::If {
                                test: JSItem::Ex { expression: Box::new(Expression::Binop {
                                    a: Box::new(Expression::Identifier {name: "b".to_string()}),
                                    op: Operator::Greater,
                                    b: Box::new(Expression::Number {value: 10.})
                                }) },
                                consequent: vec![JSItem::Ex {
                                    expression: Box::new(Expression::CallExpression {
                                        callee: Box::new(Expression::MemberExpression {
                                            object: Box::new(Expression::Identifier {name: "console".to_string()}),
                                            property: Box::new(Expression::Identifier {name: "log".to_string()})
                                        }),
                                        arguments: vec![JSItem::Ex { expression: Box::new(Expression::String {value: "greater than 10".to_string()}) }]
                                    })
                                }],
                                alternate: JSItem::Ex {expression: Box::from(Expression::None)}
                            }) }
                        })
                    }
                })
            }]
        })
    }))
}














