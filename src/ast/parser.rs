use crate::lexer::lexer::Lexer;

use crate::lexer::js_token::Tok;
use crate::ast::ast::{Operator, Expression, Statement};
use std::ops::Deref;

pub struct Parser {
    pub(crate) lexer: Lexer,
    pub ast_tree: Vec<Expression>,
    function_body: Vec<char>,
    function_tokens: Vec<Tok>,
    in_function: bool,
    function_name: String,
    function_params: Vec<Tok>,

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

    fn find_end_of_assignment(start: usize, tokens: &Vec<Tok>) -> usize {
        let mut j = start + 1;

        let mut current_type = "assignment";
        while j < tokens.len() - 1 {
            let token = tokens.get(j as usize).unwrap();

            if current_type == "assignment" {
                match token {
                    Tok::Name { name } => {
                        j += 1;
                        current_type = "name";
                    }
                    _ => {
                        return j;
                    }
                }
            } else if (current_type == "name") {
                match token {
                    Tok::Equal => {
                        j += 1;
                        current_type = "equal";
                    }
                    _ => {
                        return j;
                    }
                }
            } else if (current_type == "equal") {
                match token {
                    Tok::Name {name} => {
                        let k = Parser::find_end_of_expression(j, tokens);
                        j = k + 1;
                        current_type = "expression"
                    }
                    _ => {
                        return j;
                    }
                }
            } else if (current_type == "expression") {
                match token {
                    Tok::Lpar => {
                        let k = Parser::find_matching_paren(j, tokens);
                        j = k + 1;
                    }
                    Tok::Semi => {
                        return j;
                    }
                    _ => {
                        return j;
                    }
                }
            }
        }
        return j;
    }

    fn find_end_of_expression(start: usize, tokens: &Vec<Tok>) -> usize {
        let mut j = start + 1;

        let mut current_type = "name";
        while j < tokens.len() - 1 {
            let token = tokens.get(j as usize).unwrap();
            if current_type == "name" {
                match token {
                    Tok::Lpar => {
                        current_type = "lpar";
                        j += 1;
                    }
                    Tok::Dot => {
                        current_type = "dot";
                        j += 1;
                    }
                    Tok::Equal => {
                        current_type = "equal";
                        j += 1;
                    }
                    Tok::PlusEqual => {
                        current_type = "plus_equal";
                        j += 1;
                    }
                    _ => {
                        return j;
                    }
                }
            } else if current_type == "dot" {
                match token {
                    Tok::Name {name} => {
                        current_type = "name";
                        j += 1;
                    }
                    _ => {
                        return j;
                    }
                }
            } else if current_type == "lpar" {
                let k = Parser::find_matching_paren(j - 1, tokens);
                j = k;
                current_type = "rpar";
            } else if current_type == "rpar" {
                match token {
                    Tok::Lpar => {
                        let k = Parser::find_matching_paren(j - 1, tokens);
                        j = k;
                        current_type = "rpar";
                    }
                    Tok::Semi => {
                        return j;
                    }
                    _ => {
                        return j;
                    }
                }
            } else {
                j += 1;
            }
        }
        return j;
    }

    fn find_end_of_function(start: usize, tokens: &Vec<Tok>) -> usize {
        let mut j = start;
        let mut lbrace = 0;

        while j < tokens.len() {
            let token = tokens.get(j as usize).unwrap();
            if token.eq(&Tok::Lbrace) {
                lbrace += 1;
            } else if token.eq(&Tok::Rbrace) {
                lbrace -= 1;
                if lbrace == 0 {
                    break
                }
            }
            j += 1;
        }
        return j;
    }

    fn find_matching_paren(start: usize, tokens: &Vec<Tok>) -> usize {
        let mut j = start;
        let mut lpar = 0;
        while j < tokens.len() {
            let token = tokens.get(j as usize).unwrap();
            if token.eq(&Tok::Lpar) {
                lpar += 1;
            } else if token.eq(&Tok::Rpar) {
                lpar -= 1;
                if lpar == 0 {
                    break
                }
            }
            j += 1;
        }
        return j;
    }

    pub fn parse(&mut self, tokens: Vec<Tok>) -> Vec<Expression> {
        if tokens.len() == 1 {
            let token = tokens.get(0).unwrap();
            match token {
                Tok::Float {mut value} => {
                    return vec![Expression::Number {value}]
                }
                Tok::Name {name} => {
                    return vec![Expression::Identifier {name: name.clone() }]
                }
                _ => {

                }
            }
        }


        let mut i = 0;
        while i < tokens.len() - 1 {
            let token = tokens.get(i).unwrap();
            match token {
                Tok::Let => {
                    //assignment
                    let j = Parser::find_end_of_assignment(i, &tokens);
                    let t = tokens[i+3..=j].to_vec();
                    let mut p = Parser::new();
                    let out = p.parse(t);
                    i = j;
                }
                Tok::Name {name} => {
                    //expression
                    let j = Parser::find_end_of_expression(i+1, &tokens);
                    i = j;
                }
                Tok::Lpar => {
                    let j = Parser::find_matching_paren(i, &tokens);
                    if tokens.get(j + 1).unwrap().eq(&Tok::Lpar) {
                        //call expression
                        let j = Parser::find_matching_paren(j + 1, &tokens);
                        i = j;
                    } else {
                        //paren expression
                        i = j;
                    }
                }
                Tok::Function => {
                    //function
                    let j = Parser::find_end_of_function(i+1, &tokens);
                    i = j;
                }
                _ => {
                    i += 1;
                }
            }
        }



        // for token in tokens {
        //     self.add_token(token);
        // }
        let mut expressions = vec![];
        // loop {
        //     let ex = self.ast_tree.pop();
        //     match ex {
        //         Some(expression) => {
        //             expressions.push(expression);
        //         }
        //         None => {
        //             break
        //         }
        //     }
        // }
        // expressions.reverse();
        return expressions;
    }

    pub fn add_token(&mut self, tok: Tok) {
        match tok {
            Tok::Function => {
                if !self.in_expression {
                    self.in_function = true;
                }
                Parser::process_token(self, tok);
            }
            Tok::Lpar => {
                if !self.in_function {
                    self.in_expression = true;
                }
                Parser::process_token(self, tok);
            }
            Tok::Name {name} => {
                if !self.in_function && !self.in_block {
                    self.in_expression = true;
                }
                Parser::process_token(self, Tok::Name {name});
            }
            _ => {
                Parser::process_token(self, tok);
            }
        }
    }

    pub fn new() -> Parser {
        Parser {
            lexer: Lexer::new(),
            ast_tree: vec![],
            function_body: vec![],
            function_tokens: vec![],
            in_function: false,
            function_name: "".to_string(),
            function_params: vec![],
            expression_body: vec![],
            expression_tokens: vec![],
            in_expression: false,
            in_block: false
        }
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

    fn get_function_params(&mut self) -> Vec<Tok> {
        let mut tokens = vec![];

        loop {
            let token = self.function_params.pop();
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
                    if self.expression_tokens.len() == 1 {
                        break
                    }
                }
                None => {
                    break
                }
            }
        }
        self.expression_tokens.pop();
        tokens.reverse();
        return tokens;
    }

    fn process_token_in_function(&mut self, tok: Tok)  {
        match tok {
            Tok::Name{name} => {
                if self.in_block {
                    self.function_tokens.push(Tok::Name {name});
                } else if self.function_name == "" && self.function_params.last().unwrap().eq(&Tok::Function) {
                    self.function_name = name;
                } else {
                    self.function_params.push(Tok::Name {name});
                }
            }
            Tok::Lpar => {
                if self.in_block {
                    self.function_tokens.push(tok);
                } else {
                    self.function_params.push(tok);
                }
            }
            Tok::Rpar => {
                if self.in_block {
                    self.function_tokens.push(tok);
                } else {
                    self.function_params.push(tok);
                }
            }
            Tok::Lbrace => {
                if !self.in_block {
                    self.in_block = true;
                }
                self.function_body.push('{');
            }
            Tok::Rbrace => {
                self.function_body.pop();
                if self.function_body.len() == 0 {
                    self.in_function = false;
                    self.end_function();
                    // let function_tokens = self.get_function_tokens();
                    // let last= self.ast_tree.pop().unwrap_or(Expression::None);
                    // let ex = combine_expressions(last, self.create_function(function_tokens));
                    // self.ast_tree.push(ex);
                }
            }
            Tok::EndOfLine => {
                if self.function_tokens.len() > 0 {
                    self.function_tokens.push(tok);
                }
            }
            _ => {
                if self.in_block {
                    self.function_tokens.push(tok)
                } else {
                    self.function_params.push(tok);
                }
            }
        }
    }

    fn end_function(&mut self) {
        let function_tokens = self.get_function_tokens();
        let mut parser = Parser::new();
        let expressions = parser.parse(function_tokens);
        // let function_params = self.get_function_params();
        // let function_expression = Statement::FunctionDef {
        //     name: "".to_string(),
        //     params: vec![],
        //     body: vec![]
        // };
    }

    fn end_expression(&mut self) {
        let expression_tokens = self.get_expression_tokens();
        let mut parser = Parser::new();
        let item = parser.parse(expression_tokens);


        // let last= self.ast_tree.pop().unwrap_or(Expression::None);
    }

    fn is_call_expression_end(&mut self) {
        for i in self.expression_tokens.len()..0 {

        }
    }

    fn process_token_in_expression(&mut self, tok: Tok) {
        if tok.eq(&Tok::Lpar) {
            self.expression_body.push('(');
        } else if tok.eq(&Tok::Rpar) {
            self.expression_body.pop();
            if self.expression_body.len() == 0 {
                self.in_expression = false;
                self.end_expression();
                // let ex = combine_expressions(last, self.create_expression(expression_tokens));
                // self.ast_tree.push(ex);
            }
        }
        self.expression_tokens.push(tok);
    }

    pub fn process_token(&mut self, tok: Tok) {
        let is_in_function = self.in_function.to_owned();
        let is_in_expression = self.in_expression.to_owned();
        if is_in_function {
            self.process_token_in_function(tok);
        } else if is_in_expression {
            self.process_token_in_expression(tok);
        } else {
            let last;
            if tok.eq(&Tok::Semi)
                || tok.eq(&Tok::EndOfLine)
                || tok.eq(&Tok::Return)
                || tok.eq(&Tok::StartProgram)
            {
                return;
            } else {
                last = self.ast_tree.pop().unwrap_or(Expression::None);
            }
            let op = create_expression(last, tok);
            self.ast_tree.push(op);
        }
    }

    pub fn get_ast(&mut self) -> &Vec<Expression> {
        &self.ast_tree
    }
}