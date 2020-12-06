use crate::lexer::lexer::Lexer;
use std::fs;
use crate::parser::parser::Parser;
use crate::parser::symbols::{Expression, Operator, Statement, AssignOp};
use crate::parser::symbols::JSItem;

#[test]
fn test_simple_add() {
    let file = fs::read_to_string("js/math/simple_add.js");

    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(file.unwrap());
    let expressions = parser.parse(tokens);

    assert_eq!(expressions.len(), 1);
    let expression = expressions.get(0).unwrap();
    assert!(expression.eq(&JSItem::Ex {
        expression: Box::new(Expression::Binop {
            a: Box::new(Expression::Binop {
                a: Box::new(Expression::Number {value: 1.}),
                op: Operator::Add,
                b: Box::new(Expression::Number {value: 2.})
            }),
            op: Operator::Add,
            b: Box::new(Expression::Number {value: 3.})
        })
    }));
}

#[test]
fn test_div1() {
    let file = fs::read_to_string("js/math/div/div1.js");

    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(file.unwrap());
    let expressions = parser.parse(tokens);

    assert_eq!(expressions.len(), 1);
    let expression = expressions.get(0).unwrap();
    assert!(expression.eq(&JSItem::Ex {
        expression: Box::new(Expression::Binop {
            a: Box::new(Expression::Number {value: 1.}),
            op: Operator::Div,
            b: Box::new(Expression::Number {value: 5.})
        })
    }));
}

#[test]
fn test_div2() {
    let file = fs::read_to_string("js/math/div/div2.js");

    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(file.unwrap());
    let expressions = parser.parse(tokens);

    assert_eq!(expressions.len(), 1);
    let expression = expressions.get(0).unwrap();
    assert!(expression.eq(&JSItem::Ex {
        expression: Box::new(Expression::Binop {
            a: Box::new(Expression::Number {value: 1.}),
            op: Operator::Add,
            b: Box::new(Expression::Binop {
                a: Box::new(Expression::Number {value: 2.}),
                op: Operator::Div,
                b: Box::new(Expression::Number {value: 5.})
            })
        })
    }));
}

#[test]
fn test_div4() {
    let file = fs::read_to_string("js/math/div/div4.js");

    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(file.unwrap());
    let expressions = parser.parse(tokens);

    assert_eq!(expressions.len(), 2);
    let expression = expressions.get(0).unwrap();
    assert!(expression.eq(&JSItem::Ex {
        expression: Box::new(Expression::Binop {
            a: Box::new(Expression::Number {value: 1.}),
            op: Operator::Div,
            b: Box::new(Expression::Number {value: 2.})
        })
    }));

    let expression = expressions.get(1).unwrap();
    assert!(expression.eq(&JSItem::Ex {
        expression: Box::new(Expression::Binop {
            a: Box::new(Expression::Binop {
                a: Box::new(Expression::Number {value: 5.}),
                op: Operator::Mult,
                b: Box::new(Expression::Number {value: 3.})
            }),
            op: Operator::Add,
            b: Box::new(Expression::Binop {
                a: Box::new(Expression::Number {value: 70.}),
                op: Operator::Div,
                b: Box::new(Expression::Number {value: 10.})
            })
        })
    }));
}

#[test]
fn test_div5() {
    let file = fs::read_to_string("js/math/div/div5.js");

    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(file.unwrap());
    let expressions = parser.parse(tokens);

    assert_eq!(expressions.len(), 1);
    let expression = expressions.get(0).unwrap();
    assert!(expression.eq(&JSItem::Ex {
        expression: Box::new(Expression::Binop {
            a: Box::new(Expression::Number {value: 1.}),
            op: Operator::Add,
            b: Box::new(Expression::Binop {
                a: Box::new(Expression::SubExpression {
                    expression: Box::new(Expression::Binop {
                        a: Box::new(Expression::Number {value: 3.}),
                        op: Operator::Add,
                        b: Box::new(Expression::Number {value: 2.})
                    })
                }),
                op: Operator::Div,
                b: Box::new(Expression::Number {value: 5.})
            })
        })
    }))
}

#[test]
fn test_add_sub() {
    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(String::from("3 + 2 - 1"));
    let mut js_items = parser.parse(tokens);

    assert_eq!(js_items.len(), 1);
    let expression = js_items.get(0).unwrap();

    assert!(expression.eq(&JSItem::Ex {
        expression: Box::new(Expression::Binop {
            a: Box::new(Expression::Binop {
                a: Box::new(Expression::Number {value: 3.}),
                op: Operator::Add,
                b: Box::new(Expression::Number {value: 2.})
            }),
            op: Operator::Sub,
            b: Box::new(Expression::Number {value: 1. })
        })
    }))
}

#[test]
fn test_identifier_less_number() {
    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(String::from("a < 1"));
    let mut js_items = parser.parse(tokens);

    assert_eq!(js_items.len(), 1);
    let expression = js_items.get(0).unwrap();

    assert!(expression.eq(&JSItem::Ex {
        expression: Box::new(Expression::Binop {
            a: Box::new(Expression::Identifier {name: "a".to_string()}),
            op: Operator::Less,
            b: Box::new(Expression::Number {value: 1. })
        })
    }))
}

#[test]
fn test_number_less_number() {
    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(String::from("2 < 1"));
    let mut js_items = parser.parse(tokens);

    assert_eq!(js_items.len(), 1);
    let expression = js_items.get(0).unwrap();

    assert!(expression.eq(&JSItem::Ex {
        expression: Box::new(Expression::Binop {
            a: Box::new(Expression::Number {value: 2.}),
            op: Operator::Less,
            b: Box::new(Expression::Number {value: 1. })
        })
    }))
}

#[test]
fn test_number_less_identifier() {
    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(String::from("2 < a"));
    let mut js_items = parser.parse(tokens);

    assert_eq!(js_items.len(), 1);
    let expression = js_items.get(0).unwrap();

    assert!(expression.eq(&JSItem::Ex {
        expression: Box::new(Expression::Binop {
            a: Box::new(Expression::Number {value: 2.}),
            op: Operator::Less,
            b: Box::new(Expression::Identifier {name: "a".to_string()})
        })
    }))
}

#[test]
fn test_identifier_plus_plus() {
    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(String::from("a++"));
    let mut js_items = parser.parse(tokens);

    assert_eq!(js_items.len(), 1);
    let expression = js_items.get(0).unwrap();

    assert!(expression.eq(&JSItem::Ex {
        expression: Box::new(Expression::UpdateExpression {
            expression: Box::new(Expression::Identifier {name: "a".to_string()})
        })
    }))
}

#[test]
fn test_and1() {
    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(String::from("x == 5 && x < 10"));
    let mut js_items = parser.parse(tokens);

    assert_eq!(js_items.len(), 1);
    let expression = js_items.get(0).unwrap();

    assert!(expression.eq(&JSItem::Ex {
        expression: Box::new(Expression::Binop {
            a: Box::new(Expression::Binop {
                a: Box::new(Expression::Identifier {name: "x".to_string()}),
                op: Operator::EqEq,
                b: Box::new(Expression::Number {value: 5.})
            }),
            op: Operator::And,
            b: Box::new(Expression::Binop {
                a: Box::new(Expression::Identifier {name: "x".to_string()}),
                op: Operator::Less,
                b: Box::new(Expression::Number {value: 10.})
            })
        })
    }))
}

#[test]
fn test_triple_equal() {

    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(String::from("x === 5"));
    let mut js_items = parser.parse(tokens);

    assert_eq!(js_items.len(), 1);
    let expression = js_items.get(0).unwrap();

    assert!(expression.eq(&JSItem::Ex {
        expression: Box::new(Expression::Binop {
            a: Box::new(Expression::Identifier {name: "x".to_string()}),
            op: Operator::EqEqEq,
            b: Box::new(Expression::Number {value: 5.})
        })
    }))
}

#[test]
fn test_div_log() {
    let file = fs::read_to_string("js/math/div/div_log.js");
    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(file.unwrap());
    let mut js_items = parser.parse(tokens);

    assert_eq!(js_items.len(), 2);

    assert!(js_items.eq(&vec![JSItem::St {
        statement: Box::new(Statement::AssignmentExpression {
            operator: AssignOp::Let,
            left: JSItem::Ex { expression: Box::new(Expression::Literal { value: "b".to_string() }) },
            right: JSItem::Ex {expression: Box::new(Expression::Number {value: 5.})}
        })
    }, JSItem::Ex {
        expression: Box::new(Expression::CallExpression {
            callee: Box::new(Expression::MemberExpression {
                object: Box::new(Expression::Identifier { name: "console".to_string() }),
                property: Box::new(Expression::Identifier {name: "log".to_string()})
            }),
            arguments: vec![JSItem::Ex {
                expression: Box::new(Expression::Binop {
                    a: Box::new(Expression::Identifier {name: "b".to_string()}),
                    op: Operator::Div,
                    b: Box::new(Expression::Number {value: 2.})
                })
            }]
        })
    }]));
}
