use crate::lexer::lexer::Lexer;
use std::fs;
use crate::parser::parser::Parser;
use crate::parser::symbols::{Expression, Operator};
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