use std::fs;
use crate::lexer::lexer::Lexer;
use crate::lexer::js_token::Tok;
use crate::parser::parser::Parser;
use crate::parser::symbols::{JSItem, Statement, AssignOp, Expression, Operator};

#[test]
fn test_if1() {
    let file = fs::read_to_string("js/if_while_for/if1.js");

    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(file.unwrap());
    let js_items = parser.parse(tokens);

    assert_eq!(js_items.len(), 2);
    let assign = js_items.get(0).unwrap();
    let ist = js_items.get(1).unwrap();
    assert!(assign.eq(&JSItem::St {
        statement: Box::new(Statement::AssignmentExpression {
            operator: AssignOp::Const,
            left: JSItem::Ex {
                expression: Box::new(Expression::Literal { value: "x".to_string() })
            },
            right: JSItem::Ex {
                expression: Box::new(Expression::Number { value: 5. })
            }
        })
    }));

    assert!(ist.eq(&JSItem::St {
        statement: Box::new(Statement::If {
            test: JSItem::Ex {
                expression: Box::new(Expression::Binop {
                    a: Box::new(Expression::Identifier {name: "x".to_string()}),
                    op: Operator::Greater,
                    b: Box::new(Expression::Number {value: 2.})
                })
            },
            consequent: vec![JSItem::Ex {
                expression: Box::new(Expression::CallExpression {
                    callee: Box::new(Expression::MemberExpression {
                        object: Box::new(Expression::Identifier {name: "console".to_string()}),
                        property: Box::new(Expression::Identifier {name: "log".to_string()})
                    }),
                    arguments: vec![JSItem::Ex {
                        expression: Box::new(Expression::String {value: "hi".to_string()})
                    }]
                })
            }],
            alternate: JSItem::Ex { expression: Box::from(Expression::None)}
        })
    }));
}

#[test]
fn test_if2() {
    let file = fs::read_to_string("js/if_while_for/if2.js");

    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(file.unwrap());
    let js_items = parser.parse(tokens);

    assert_eq!(js_items.len(), 2);
    let assign = js_items.get(0).unwrap();
    let ist = js_items.get(1).unwrap();

    assert!(assign.eq(&JSItem::St {
        statement: Box::new(Statement::AssignmentExpression {
            operator: AssignOp::Const,
            left: JSItem::Ex {
                expression: Box::new(Expression::Literal { value: "x".to_string() })
            },
            right: JSItem::Ex {
                expression: Box::new(Expression::Number { value: 5. })
            }
        })
    }));

    assert!(ist.eq(&JSItem::St {
        statement: Box::new(Statement::If {
            test: JSItem::Ex {
                expression: Box::new(Expression::Binop {
                    a: Box::new(Expression::Identifier {name: "x".to_string()}),
                    op: Operator::Greater,
                    b: Box::new(Expression::Number {value: 2.})
                })
            },
            consequent: vec![JSItem::Ex {
                expression: Box::new(Expression::CallExpression {
                    callee: Box::new(Expression::MemberExpression {
                        object: Box::new(Expression::Identifier {name: "console".to_string()}),
                        property: Box::new(Expression::Identifier {name: "log".to_string()})
                    }),
                    arguments: vec![JSItem::Ex {
                        expression: Box::new(Expression::String {value: "hi".to_string()})
                    }]
                })
            }],
            alternate: JSItem::St {
                statement: Box::new(Statement::If {
                    test: JSItem::Ex {
                        expression: Box::new(Expression::Binop {
                            a: Box::new(Expression::Identifier {name: "x".to_string()}),
                            op: Operator::Less,
                            b: Box::new(Expression::Number {value: 2.})
                        })
                    },
                    consequent: vec![JSItem::Ex {
                        expression: Box::new(Expression::CallExpression {
                            callee: Box::new(Expression::MemberExpression {
                                object: Box::new(Expression::Identifier {name: "console".to_string()}),
                                property: Box::new(Expression::Identifier {name: "log".to_string()})
                            }),
                            arguments: vec![JSItem::Ex {
                                expression: Box::new(Expression::String {value: "gt 2".to_string()})
                            }]
                        })
                    }],
                    alternate: JSItem::Ex {
                        expression: Box::new(Expression::CallExpression {
                            callee: Box::new(Expression::MemberExpression {
                                object: Box::new(Expression::Identifier {name: "console".to_string()}),
                                property: Box::new(Expression::Identifier {name: "log".to_string()})
                            }),
                            arguments: vec![JSItem::Ex {
                                expression: Box::new(Expression::String {value: "there".to_string()})
                            }]
                        })
                    }
                })
            }
        })
    }))
}

#[test]
fn test_if3() {
    let file = fs::read_to_string("js/if_while_for/if3.js");

    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(file.unwrap());
    let js_items = parser.parse(tokens);

    assert_eq!(js_items.len(), 2);
    let assign = js_items.get(0).unwrap();
    let ist = js_items.get(1).unwrap();

    assert!(assign.eq(&JSItem::St {
        statement: Box::new(Statement::AssignmentExpression {
            operator: AssignOp::Const,
            left: JSItem::Ex {
                expression: Box::new(Expression::Literal { value: "x".to_string() })
            },
            right: JSItem::Ex {
                expression: Box::new(Expression::Number { value: 5. })
            }
        })
    }));

    assert!(ist.eq(&JSItem::St {
        statement: Box::new(Statement::If {
            test: JSItem::Ex {
                expression: Box::new(Expression::Binop {
                    a: Box::new(Expression::Identifier {name: "x".to_string()}),
                    op: Operator::Greater,
                    b: Box::new(Expression::Number {value: 2.})
                })
            },
            consequent: vec![JSItem::Ex {
                expression: Box::new(Expression::CallExpression {
                    callee: Box::new(Expression::MemberExpression {
                        object: Box::new(Expression::Identifier {name: "console".to_string()}),
                        property: Box::new(Expression::Identifier {name: "log".to_string()})
                    }),
                    arguments: vec![JSItem::Ex {
                        expression: Box::new(Expression::String {value: "hi".to_string()})
                    }]
                })
            }],
            alternate: JSItem::St {
                statement: Box::new(Statement::If {
                    test: JSItem::Ex {
                        expression: Box::new(Expression::Binop {
                            a: Box::new(Expression::Identifier {name: "x".to_string()}),
                            op: Operator::Less,
                            b: Box::new(Expression::Number {value: 2.})
                        })
                    },
                    consequent: vec![JSItem::Ex {
                        expression: Box::new(Expression::CallExpression {
                            callee: Box::new(Expression::MemberExpression {
                                object: Box::new(Expression::Identifier {name: "console".to_string()}),
                                property: Box::new(Expression::Identifier {name: "log".to_string()})
                            }),
                            arguments: vec![JSItem::Ex {
                                expression: Box::new(Expression::String {value: "gt 2".to_string()})
                            }]
                        })
                    }],
                    alternate: JSItem::Ex {
                        expression: Box::new(Expression::CallExpression {
                            callee: Box::new(Expression::MemberExpression {
                                object: Box::new(Expression::Identifier {name: "console".to_string()}),
                                property: Box::new(Expression::Identifier {name: "log".to_string()})
                            }),
                            arguments: vec![JSItem::Ex {
                                expression: Box::new(Expression::String {value: "there".to_string()})
                            }]
                        })
                    }
                })
            }
        })
    }))
}

#[test]
fn test_if3_1() {
    let file = fs::read_to_string("js/if_while_for/if3_1.js");

    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(file.unwrap());
    let js_items = parser.parse(tokens);

    assert_eq!(js_items.len(), 2);
    let assign = js_items.get(0).unwrap();
    let ist = js_items.get(1).unwrap();

    assert!(assign.eq(&JSItem::St {
        statement: Box::new(Statement::AssignmentExpression {
            operator: AssignOp::Const,
            left: JSItem::Ex {
                expression: Box::new(Expression::Literal { value: "x".to_string() })
            },
            right: JSItem::Ex {
                expression: Box::new(Expression::Number { value: 5. })
            }
        })
    }));

    assert!(ist.eq(&JSItem::St {
        statement: Box::new(Statement::If {
            test: JSItem::Ex {
                expression: Box::new(Expression::Binop {
                    a: Box::new(Expression::Identifier {name: "x".to_string()}),
                    op: Operator::EqEq,
                    b: Box::new(Expression::Number {value: 2.})
                })
            },
            consequent: vec![JSItem::Ex {
                expression: Box::new(Expression::CallExpression {
                    callee: Box::new(Expression::MemberExpression {
                        object: Box::new(Expression::Identifier {name: "console".to_string()}),
                        property: Box::new(Expression::Identifier {name: "log".to_string()})
                    }),
                    arguments: vec![JSItem::Ex {
                        expression: Box::new(Expression::String {value: "hi".to_string()})
                    }]
                })
            }],
            alternate: JSItem::St {
                statement: Box::new(Statement::If {
                    test: JSItem::Ex {
                        expression: Box::new(Expression::Binop {
                            a: Box::new(Expression::Identifier {name: "x".to_string()}),
                            op: Operator::Less,
                            b: Box::new(Expression::Number {value: 2.})
                        })
                    },
                    consequent: vec![JSItem::Ex {
                        expression: Box::new(Expression::CallExpression {
                            callee: Box::new(Expression::MemberExpression {
                                object: Box::new(Expression::Identifier {name: "console".to_string()}),
                                property: Box::new(Expression::Identifier {name: "log".to_string()})
                            }),
                            arguments: vec![JSItem::Ex {
                                expression: Box::new(Expression::String {value: "lt 2".to_string()})
                            }]
                        })
                    }],
                    alternate: JSItem::Ex {
                        expression: Box::new(Expression::CallExpression {
                            callee: Box::new(Expression::MemberExpression {
                                object: Box::new(Expression::Identifier {name: "console".to_string()}),
                                property: Box::new(Expression::Identifier {name: "log".to_string()})
                            }),
                            arguments: vec![JSItem::Ex {
                                expression: Box::new(Expression::String {value: "there".to_string()})
                            }]
                        })
                    }
                })
            }
        })
    }))
}

#[test]
fn test_if3_2() {
    let file = fs::read_to_string("js/if_while_for/if3_2.js");

    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(file.unwrap());
    let js_items = parser.parse(tokens);

    assert_eq!(js_items.len(), 2);
    let assign = js_items.get(0).unwrap();
    let ist = js_items.get(1).unwrap();

    assert!(assign.eq(&JSItem::St {
        statement: Box::new(Statement::AssignmentExpression {
            operator: AssignOp::Const,
            left: JSItem::Ex {
                expression: Box::new(Expression::Literal { value: "x".to_string() })
            },
            right: JSItem::Ex {
                expression: Box::new(Expression::Number { value: 3. })
            }
        })
    }));

    assert!(ist.eq(&JSItem::St {
        statement: Box::new(Statement::If {
            test: JSItem::Ex {
                expression: Box::new(Expression::Binop {
                    a: Box::new(Expression::Identifier {name: "x".to_string()}),
                    op: Operator::EqEqEq,
                    b: Box::new(Expression::Number {value: 2.})
                })
            },
            consequent: vec![JSItem::Ex {
                expression: Box::new(Expression::CallExpression {
                    callee: Box::new(Expression::MemberExpression {
                        object: Box::new(Expression::Identifier {name: "console".to_string()}),
                        property: Box::new(Expression::Identifier {name: "log".to_string()})
                    }),
                    arguments: vec![JSItem::Ex {
                        expression: Box::new(Expression::String {value: "hi".to_string()})
                    }]
                })
            }],
            alternate: JSItem::St {
                statement: Box::new(Statement::If {
                    test: JSItem::Ex {
                        expression: Box::new(Expression::Binop {
                            a: Box::new(Expression::Identifier {name: "x".to_string()}),
                            op: Operator::Less,
                            b: Box::new(Expression::Number {value: 2.})
                        })
                    },
                    consequent: vec![JSItem::Ex {
                        expression: Box::new(Expression::CallExpression {
                            callee: Box::new(Expression::MemberExpression {
                                object: Box::new(Expression::Identifier {name: "console".to_string()}),
                                property: Box::new(Expression::Identifier {name: "log".to_string()})
                            }),
                            arguments: vec![JSItem::Ex {
                                expression: Box::new(Expression::String {value: "lt 2".to_string()})
                            }]
                        })
                    }],
                    alternate: JSItem::Ex {
                        expression: Box::new(Expression::CallExpression {
                            callee: Box::new(Expression::MemberExpression {
                                object: Box::new(Expression::Identifier {name: "console".to_string()}),
                                property: Box::new(Expression::Identifier {name: "log".to_string()})
                            }),
                            arguments: vec![JSItem::Ex {
                                expression: Box::new(Expression::String {value: "there".to_string()})
                            }]
                        })
                    }
                })
            }
        })
    }))
}