use crate::lexer::lexer::Lexer;

use crate::lexer::js_token::Tok;
use crate::ast::ast::{Operator, Expression};
use std::ops::Deref;

pub struct Parser {
    pub(crate) lexer: Lexer,
    pub ast_tree: Vec<Expression>,
    function_body: Vec<char>,
    function_tokens: Vec<Tok>,
    in_function: bool,

    expression_body: Vec<char>,
    expression_tokens: Vec<Tok>,
    in_expression: bool,
    in_block: bool
}

fn create_expression(last: Expression, tok: Tok) -> Expression {
    match tok {
        Tok::Float{value} => {
            match last {
                Expression::Binop {a, op, b} => {
                    let last = *b;
                    let next = create_expression(last, tok);
                    return Expression::Binop {a, op, b: Box::new(next) };
                }
                _ => {
                    return Expression::Number {value};
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

fn combine_expressions(last: Expression, next: Expression) -> Expression {
    match last {
        Expression::Binop { a, op, b } => {
            return Expression::Binop {a, op, b: Box::new(next) };
        }
        _ => {
            return next;
        }
    }
    return Expression::None;
}


impl Parser {

    pub fn new() -> Parser {
        Parser {
            lexer: Lexer::new(),
            ast_tree: vec![],
            function_body: vec![],
            function_tokens: vec![],
            in_function: false,
            expression_body: vec![],
            expression_tokens: vec![],
            in_expression: false,
            in_block: false
        }
    }

    fn create_function(&mut self, tokens: Vec<Tok>) -> Expression {
        let mut parser = Parser::new();
        let mut ast = parser.parse(tokens);
        let mut boxed_expressions = vec![];
        for ex in ast {
            boxed_expressions.push(Box::new(ex))
        }
        return Expression::Function {ops: boxed_expressions}
    }

    fn create_expression(&mut self, tokens: Vec<Tok>) -> Expression {
        let mut parser = Parser::new();
        let mut ast = parser.parse(tokens);
        let mut boxed_expressions = vec![];
        for ex in ast {
            boxed_expressions.push(Box::new(ex))
        }
        return Expression::Function {ops: boxed_expressions}
    }

    fn get_function_tokens(&mut self) -> Vec<Tok> {
        let mut tokens = vec![];

        loop {
            let token = self.function_tokens.pop();
            match token {
                Some(token) => {
                    tokens.push(token);
                }
                None => {
                    break
                }
            }
        }
        tokens.reverse();
        return tokens;
    }

    fn get_expression_tokens(&mut self) -> Vec<Tok> {
        let mut tokens = vec![];

        loop {
            let token = self.expression_tokens.pop();
            match token {
                Some(token) => {
                    tokens.push(token);
                }
                None => {
                    break
                }
            }
        }
        tokens.reverse();
        return tokens;
    }

    fn process_token_in_function(&mut self, tok: Tok)  {
        if tok.eq(&Tok::Lpar) {
            self.function_body.push('(');
        } else if tok.eq(&Tok::Rpar) {
            self.function_body.pop();
        } else if tok.eq(&Tok::Lbrace) {
            self.function_body.push('{');
        } else if tok.eq(&Tok::Rbrace) {
            self.function_body.pop();
            if self.function_body.len() == 0 {
                self.in_function = false;
                let function_tokens = self.get_function_tokens();
                let last= self.ast_tree.pop().unwrap_or(Expression::None);
                let ex = combine_expressions(last, self.create_function(function_tokens));
                self.ast_tree.push(ex);
            }
        } else {
            self.function_tokens.push(tok)
        }
    }

    fn process_token_in_expression(&mut self, tok: Tok) {
        if tok.eq(&Tok::Lpar) {
            self.expression_body.push('(');
        } else if tok.eq(&Tok::Rpar) {
            self.expression_body.pop();
            if self.expression_body.len() == 0 {
                self.in_expression = false;
                let expression_tokens = self.get_expression_tokens();
                let last= self.ast_tree.pop().unwrap_or(Expression::None);
                let ex = combine_expressions(last, self.create_expression(expression_tokens));
                self.ast_tree.push(ex);
            }
        }
    }

    pub fn process_token(&mut self, tok: Tok) {
        let is_in_function = self.in_function.to_owned();
        if is_in_function {
            self.process_token_in_function(tok);
        } else {
            let last;
            if tok.eq(&Tok::Semi)
                || tok.eq(&Tok::EndOfLine)
                || tok.eq(&Tok::Return)
            {
                return;
            } else {
                last = self.ast_tree.pop().unwrap_or(Expression::None);
            }
            let op = create_expression(last, tok);
            self.ast_tree.push(op);
        }
    }

    pub fn add_token(&mut self, tok: Tok) {
        if tok.eq(&Tok::Function) && !self.in_expression {
            self.in_function = true;
        } else if tok.eq(&Tok::Lpar) && !self.in_function {
            self.in_expression = true;
        } else {
            Parser::process_token(self, tok);
        }
    }

    pub fn get_ast(&mut self) -> &Vec<Expression> {
        &self.ast_tree
    }

    pub fn parse(&mut self, tokens: Vec<Tok>) -> Vec<Expression> {
        for token in tokens {
            self.add_token(token);
        }
        let mut expressions = vec![];
        loop {
            let ex = self.ast_tree.pop();
            match ex {
                Some(expression) => {
                    expressions.push(expression);
                }
                None => {
                    break
                }
            }
        }
        expressions.reverse();
        return expressions;
    }
}