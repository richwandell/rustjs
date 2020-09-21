use crate::lexer::lexer::Lexer;
use std::fs;
use crate::parser::parser::Parser;
use crate::parser::symbols::{Expression, Statement, Operator, AssignOp};
use crate::parser::symbols::JSItem;
use crate::lexer::js_token::Tok;
use std::collections::HashMap;

#[test]
fn test_number_array() {
    let file = fs::read_to_string("js/arrays/number_array.js");
    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(file.unwrap());
    let js_items = parser.parse(tokens);

    assert_eq!(js_items.len(), 1);
    let function = js_items.get(0).unwrap();
    assert!(function.eq(&JSItem::St {
        statement: Box::new(Statement::AssignExpression {
            assign_op: AssignOp::Let,
            name: "a".to_string(),
            value: Box::new(Expression::ArrayExpression {
                items: vec![
                    JSItem::Ex { expression: Box::new(Expression::Number {value: 1.}) },
                    JSItem::Ex { expression: Box::new(Expression::Number {value: 2.}) },
                    JSItem::Ex { expression: Box::new(Expression::Number {value: 3.}) },
                    JSItem::Ex { expression: Box::new(Expression::Number {value: 4.}) },
                    JSItem::Ex { expression: Box::new(Expression::Number {value: 5.}) },
                    JSItem::Ex { expression: Box::new(Expression::Number {value: 6.}) },
                    JSItem::Ex { expression: Box::new(Expression::Number {value: 7.}) },
                    JSItem::Ex { expression: Box::new(Expression::Number {value: 8.}) },
                    JSItem::Ex { expression: Box::new(Expression::Number {value: 9.}) },
                    JSItem::Ex { expression: Box::new(Expression::Number {value: 10.}) },
                    JSItem::Ex { expression: Box::new(Expression::Number {value: 11.}) }
                ]
            })
        })
    }))
}

#[test]
fn test_array_object_apply() {
    let file = fs::read_to_string("js/arrays/array_object_apply.js");
    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(file.unwrap());
    let js_items = parser.parse(tokens);
    assert_eq!(js_items.len(), 1);
    let object = js_items.get(0).unwrap();
    let mut object_properties = HashMap::new();
    object_properties.insert("length".to_string(), JSItem::Ex {
        expression: Box::from(Expression::Number {value: 50.})
    });
    assert!(object.eq(&JSItem::St {
        statement: Box::new(Statement::AssignExpression {
            assign_op: AssignOp::Const,
            name: "real_numbers".to_string(),
            value: Box::new(Expression::CallExpression {
                callee: Box::new(Expression::MemberExpression {
                    object: Box::new(Expression::CallExpression {
                        callee: Box::new(Expression::MemberExpression {
                            object: Box::new(Expression::Identifier {name: "Array".to_string()}),
                            property: Box::new(Expression::Identifier {name: "apply".to_string()})
                        }),
                        arguments: vec![
                            JSItem::Ex {expression: Box::from(Expression::Null)},
                            JSItem::Object {mutable: true, properties: object_properties}
                        ]
                    }),
                    property: Box::new(Expression::Identifier {name: "map".to_string()})
                }),
                arguments: vec![JSItem::Ex {
                    expression: Box::from(Expression::FuncEx {
                        params: vec![
                            Tok::Name {name: "i".to_string()},
                            Tok::Name {name: "current".to_string()}
                        ],
                        body: vec![JSItem::St {
                            statement: Box::from(Statement::Return {
                                value: Box::new(JSItem::Ex {
                                    expression: Box::new(Expression::Identifier {name: "current".to_string()})
                                })
                            })
                        }]
                    })
                }]
            })
        })
    }));
}