use crate::lexer::lexer::Lexer;

use crate::lexer::js_token::Tok;
use crate::ast::ast::{Operator, Expression};
use std::ops::Deref;

pub struct Parser {
    pub(crate) lexer: Lexer,
    pub ast_tree: Vec<Expression>
}

fn create_expression(last: Expression, tok: &Tok) -> Expression {
    match tok {
        Tok::Float{value} => {
            match last {
                Expression::Binop {a, op, b} => {
                    let last = *b;
                    let next = create_expression(last, tok);
                    return Expression::Binop {a, op, b: Box::new(next) };
                }
                _ => {
                    return Expression::Number {value: value.clone()};
                }
            }
        }
        Tok::Plus => {
            return Expression::Binop {
                a: Box::new(last),
                op: Operator::Add,
                b: Box::new(Expression::None)
            }
        }
        Tok::Star => {
            match last {
                Expression::Binop {a, op, b} => {
                    return Expression::Binop {
                        a,
                        op,
                        b: Box::from(Expression::Binop {
                            a: b,
                            op: Operator::Mult,
                            b: Box::new(Expression::None)
                        })
                    };
                }
                _ => {
                    return Expression::Binop {
                        a: Box::new(last),
                        op: Operator::Mult,
                        b: Box::new(Expression::None)
                    }
                }
            }
        }
        Tok::Bslash => {
            match last {
                Expression::Binop {a, op, b} => {
                    return Expression::Binop {
                        a,
                        op,
                        b: Box::from(Expression::Binop {
                            a: b,
                            op: Operator::Div,
                            b: Box::new(Expression::None)
                        })
                    };
                }
                _ => {
                    return Expression::Binop {
                        a: Box::new(last),
                        op: Operator::Div,
                        b: Box::new(Expression::None)
                    }
                }
            }
        }
        _ => {
            return Expression::None;
        }
    }
}

impl Parser {

    pub fn new() -> Parser {
        Parser { lexer: Lexer::new(), ast_tree: vec![] }
    }

    pub fn process_token(ast_tree: &mut Vec<Expression>, tok: &Tok) {
        let last;
        if tok.eq(&Tok::Semi) || tok.eq(&Tok::EndOfLine) {
            last = Expression::None;
        } else {
            last = ast_tree.pop().unwrap_or(Expression::None);
        }
        let op = create_expression(last, tok);
        ast_tree.push(op);
    }

    pub fn add_token(&mut self, tok: &Tok) {
        Parser::process_token(&mut self.ast_tree, tok);
    }

    pub fn parse(&mut self, lex: Lexer) -> &Vec<Expression> {
        let mut iter = lex.tokens.iter();
        loop {
            let token = iter.next();

            match token {
                Some(tok) => {
                    Parser::process_token(&mut self.ast_tree, tok)
                }
                None => {
                    break
                }
            }
        }
        return &self.ast_tree;
    }
}