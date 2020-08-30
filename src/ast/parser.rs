use crate::lexer::lexer::Lexer;

use crate::lexer::js_token::Tok;
use crate::ast::ast::{Operator, Expression, Statement, JSItem};
use std::ops::Deref;
use std::borrow::Borrow;
use crate::ast::ast::Expression::{CallExpression, Identifier};

pub struct Parser {
    pub(crate) lexer: Lexer,
    pub ast_tree: Vec<Expression>,
}

fn create_expression(last: Expression, tok: Tok) -> Expression {
    match tok {
        Tok::Float { value } => {
            match last {
                Expression::Binop { a, op, b } => {
                    let last = *b;
                    let next = create_expression(last, tok);
                    return Expression::Binop { a, op, b: Box::new(next) };
                }
                _ => {
                    return Expression::Number { value };
                }
            }
        }
        Tok::Plus => {
            return Expression::Binop {
                a: Box::new(last),
                op: Operator::Add,
                b: Box::new(Expression::None),
            };
        }
        Tok::Star => {
            match last {
                Expression::Binop { a, op, b } => {
                    return Expression::Binop {
                        a,
                        op,
                        b: Box::from(Expression::Binop {
                            a: b,
                            op: Operator::Mult,
                            b: Box::new(Expression::None),
                        }),
                    };
                }
                _ => {
                    return Expression::Binop {
                        a: Box::new(last),
                        op: Operator::Mult,
                        b: Box::new(Expression::None),
                    };
                }
            }
        }
        Tok::Bslash => {
            match last {
                Expression::Binop { a, op, b } => {
                    return Expression::Binop {
                        a,
                        op,
                        b: Box::from(Expression::Binop {
                            a: b,
                            op: Operator::Div,
                            b: Box::new(Expression::None),
                        }),
                    };
                }
                _ => {
                    return Expression::Binop {
                        a: Box::new(last),
                        op: Operator::Div,
                        b: Box::new(Expression::None),
                    };
                }
            }
        }
        _ => {
            return Expression::None;
        }
    }
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
                    Tok::Name { name } => {
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

        let mut prev_type = "name";
        while j < tokens.len() - 1 {
            let token = tokens.get(j as usize).unwrap();
            if prev_type == "name" {
                match token {
                    Tok::Lpar => {
                        prev_type = "lpar";
                        j += 1;
                    }
                    Tok::Dot => {
                        prev_type = "dot";
                        j += 1;
                    }
                    Tok::Equal => {
                        prev_type = "equal";
                        j += 1;
                    }
                    Tok::PlusEqual => {
                        prev_type = "plus_equal";
                        j += 1;
                    }
                    _ => {
                        return j;
                    }
                }
            } else if prev_type == "dot" {
                match token {
                    Tok::Name { name } => {
                        prev_type = "name";
                        j += 1;
                    }
                    _ => {
                        return j;
                    }
                }
            } else if prev_type == "lpar" {
                let k = Parser::find_matching_paren(j - 1, tokens);
                j = k + 1;
                prev_type = "rpar";
            } else if prev_type == "rpar" {
                match token {
                    Tok::Lpar => {
                        let k = Parser::find_matching_paren(j - 1, tokens);
                        j = k;
                        prev_type = "rpar";
                    }
                    Tok::EndOfLine => {
                        return j - 1;
                    }
                    Tok::Dot => {
                        prev_type = "dot";
                        j += 1;
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
                    break;
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
                    break;
                }
            }
            j += 1;
        }
        return j;
    }

    pub fn parse(&mut self, tokens: Vec<Tok>) -> Vec<JSItem> {
        if tokens.len() == 1 {
            let token = tokens.get(0).unwrap();
            match token {
                Tok::Float { mut value } => {
                    return vec![JSItem::Ex { expression: Box::new(Expression::Number { value }) }];
                }
                Tok::Name { name } => {
                    return vec![JSItem::Ex { expression: Box::new(Expression::Identifier { name: name.clone() }) }];
                }
                _ => {}
            }
        }

        let mut js_items = vec![];
        let mut i = 0;
        while i < tokens.len() - 1 {
            let token = tokens.get(i).unwrap();
            match token {
                Tok::Let => {
                    //assignment
                    let j = Parser::find_end_of_assignment(i, &tokens);
                    let t = tokens[i + 3..=j].to_vec();
                    let mut p = Parser::new();
                    let out = p.parse(t);
                    i = j;
                }
                Tok::Name { name } => {
                    //expression
                    let j = Parser::find_end_of_expression(i, &tokens);
                    let mut t = tokens[i..=j].to_vec();
                    let ex = self.create_expression(t);
                    js_items.push(ex);
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
                    let j = Parser::find_end_of_function(i + 1, &tokens);
                    let mut t = tokens[i..=j].to_vec();
                    let func = self.create_function(t);
                    js_items.push(func);
                    i = j;
                }
                _ => {
                    i += 1;
                }
            }
        }

        return js_items;
    }

    fn combine_dot(last_exp: Expression, tok: Tok) -> Expression {
        match last_exp {
            Expression::CallExpression { callee, arguments } => {
                match *callee {
                    Expression::Identifier { name } => {
                        return Expression::CallExpression {
                            callee: Box::new(Expression::MemberExpression {
                                object: Box::new(Expression::None),
                                property: Box::new(Expression::Identifier { name }),
                            }),
                            arguments,
                        };
                    }
                    Expression::MemberExpression { object, property } => {
                        let new_object = Parser::combine_dot(*object, tok);

                        return Expression::CallExpression {
                            callee: Box::new(Expression::MemberExpression {
                                object: Box::from(new_object),
                                property
                            }),
                            arguments
                        };
                    }
                    _ => {}
                }
            }
            Expression::MemberExpression { object, property } => {
                match *object {
                    Expression::Identifier {name} => {
                        let new_object = Parser::combine_dot(Expression::Identifier {name}, tok);
                        return Expression::MemberExpression {
                            object: Box::from(new_object),
                            property
                        }
                    }
                    Expression::None => {

                    }
                    Expression::MemberExpression {object, property} => {
                        let new_object = Parser::combine_dot(*object, tok);
                        let new_expression = Expression::MemberExpression {
                            object: Box::new(new_object),
                            property
                        };
                        return new_expression;
                    }
                    _ => {}
                }
            }
            Expression::Identifier { name } => {
                return Expression::MemberExpression {
                    object: Box::new(Expression::None),
                    property: Box::new(Identifier {name})
                }
            }
            _ => {}
        }
        return Expression::None;
    }

    fn combine_name(last_exp: Expression, name: String) -> Expression {
        match last_exp {
            Expression::CallExpression { callee, arguments } => {
                match *callee {
                    Expression::None => {
                        return Expression::CallExpression {
                            callee: Box::new(Expression::Identifier { name }),
                            arguments
                        };
                    }
                    Expression::MemberExpression { object, property } => {
                        let new_object = Parser::combine_name(*object, name);

                        return Expression::CallExpression {
                            callee: Box::new(Expression::MemberExpression {
                                object: Box::from(new_object),
                                property
                            }),
                            arguments
                        };
                    }
                    _ => {}
                }
            }
            Expression::Identifier { name } => {
                return Expression::MemberExpression {
                    object: Box::new(Expression::None),
                    property: Box::new(Identifier {name})
                }
            }
            Expression::None => {
                return Expression::Identifier {name};
            }
            Expression::MemberExpression { object, property } => {
                let outer_property = property;
                match *object {
                    Expression::None => {
                        return Expression::MemberExpression {
                            object: Box::from(Parser::combine_name(Expression::None, name)),
                            property: outer_property
                        }
                    }
                    Expression::MemberExpression {object, property} => {
                        return Expression::MemberExpression {
                            object: Box::from(Expression::MemberExpression {
                                object: Box::new(Parser::combine_name(*object, name)),
                                property
                            }),
                            property: outer_property
                        };
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        return Expression::None;
    }

    fn combine_call(last_exp: Expression, params: Vec<Tok>) -> Expression {
        match last_exp {
            Expression::None => {
                return Expression::CallExpression {
                    callee: Box::new(Expression::None),
                    arguments: params
                };
            }
            Expression::CallExpression { callee, arguments } => {
                match *callee {
                    Expression::None => {
                        return Expression::CallExpression {
                            callee: Box::new(Expression::CallExpression {
                                callee: Box::new(Expression::None),
                                arguments: params
                            }),
                            arguments
                        };
                    }
                    Expression::MemberExpression { object, property } => {
                        let new_object = Parser::combine_call(*object, params);

                        return Expression::CallExpression {
                            callee: Box::new(Expression::MemberExpression {
                                object: Box::from(new_object),
                                property
                            }),
                            arguments
                        };
                    }
                    _ => {}
                }
            }
            Expression::Identifier { name } => {
                return Expression::MemberExpression {
                    object: Box::new(Expression::None),
                    property: Box::new(Identifier {name})
                }
            }
            Expression::MemberExpression { object, property } => {
                let outer_property = property;
                match *object {
                    Expression::None => {
                        return Expression::MemberExpression {
                            object: Box::from(Parser::combine_call(Expression::None, params)),
                            property: outer_property
                        }
                    }
                    Expression::MemberExpression {object, property} => {
                        return Expression::MemberExpression {
                            object: Box::from(Expression::MemberExpression {
                                object: Box::new(Parser::combine_call(*object, params)),
                                property
                            }),
                            property: outer_property
                        };
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        return Expression::None;
    }

    fn create_expression(&mut self, mut tokens: Vec<Tok>) -> JSItem {
        //find out if we have a call expression
        let mut call_expression_params_stack = vec![];
        let mut in_call_expression_params = false;
        let mut call_expression_params = vec![];
        let mut expression_stack = vec![];
        while tokens.len() > 0 {
            let token = tokens.pop().unwrap();
            match token {
                Tok::Dot => {
                    let ex = expression_stack.pop().unwrap();
                    let exp = Parser::combine_dot(ex, token);
                    expression_stack.push(exp);
                }
                Tok::Name { name } => {
                    if in_call_expression_params {
                        call_expression_params.push(Tok::Name { name });
                    } else {
                        let ex = expression_stack.pop().unwrap();
                        let exp = Parser::combine_name(ex, name);
                        expression_stack.push(exp);
                    }
                }
                Tok::Semi | Tok::EndOfLine => {}
                Tok::Comma => {}
                Tok::Rpar => {
                    if !in_call_expression_params {
                        call_expression_params_stack.push(")");
                        in_call_expression_params = true;
                    }
                }
                Tok::Lpar => {
                    if in_call_expression_params {
                        call_expression_params_stack.pop();
                        if call_expression_params_stack.is_empty() {
                            if expression_stack.len() > 0 {
                                let ex = expression_stack.pop().unwrap();
                                call_expression_params.reverse();
                                let exp = Parser::combine_call(ex, call_expression_params.clone());
                                expression_stack.push(exp);
                            } else {
                                call_expression_params.reverse();
                                expression_stack.push(Expression::CallExpression {
                                    callee: Box::new(Expression::None),
                                    arguments: call_expression_params.clone(),
                                });
                            }
                            call_expression_params.clear();
                            in_call_expression_params = false;
                        } else {
                            call_expression_params.push(token);
                        }
                    }
                    //TODO: do something with other tokens
                    else {}
                }
                _ => {
                    if in_call_expression_params {
                        call_expression_params.push(token);
                    }
                }
            }
        }
        let expression = Box::new(expression_stack.pop().unwrap());
        return JSItem::Ex { expression };
    }

    fn create_function(&mut self, mut tokens: Vec<Tok>) -> JSItem {
        tokens.reverse();

        //get rid of function
        tokens.pop();
        //function name should be next
        let mut function_name = String::from("");
        match tokens.pop().unwrap() {
            Tok::Name { name } => {
                function_name = name;
            }
            _ => {}
        }
        let mut function_params = vec![];
        let mut function_body = vec![];
        let mut stack = vec![];
        let mut in_params = true;
        let mut in_body = false;
        while tokens.len() > 0 {
            let token = tokens.pop().unwrap();
            match token {
                Tok::Lpar => {
                    if !in_body {
                        stack.push("(");
                        in_params = true;
                    } else {
                        function_body.push(token);
                    }
                }
                Tok::Rpar => {
                    if in_params {
                        stack.pop();
                        if stack.is_empty() {
                            in_params = false;
                        }
                    } else {
                        function_body.push(token);
                    }
                }
                Tok::Lbrace => {
                    if stack.is_empty() {
                        in_body = true;
                    } else {
                        function_body.push(token);
                    }
                    stack.push("{");
                }
                Tok::Rbrace => {
                    if in_body {
                        stack.pop();
                        if stack.is_empty() {
                            in_body = false;
                        } else {
                            function_body.push(token);
                        }
                    }
                }
                Tok::Comma => {
                    if !in_params {
                        function_body.push(token);
                    }
                }
                _ => {
                    if in_params {
                        function_params.push(token);
                    } else if in_body {
                        function_body.push(token);
                    }
                }
            }
        }

        let mut p = Parser::new();
        let out = p.parse(function_body);

        let statement = Box::new(Statement::FunctionDef {
            name: function_name,
            params: function_params,
            body: out,
        });
        let item = JSItem::St { statement };
        return item;
    }

    pub fn new() -> Parser {
        Parser {
            lexer: Lexer::new(),
            ast_tree: vec![]
        }
    }

    pub fn get_ast(&mut self) -> &Vec<Expression> {
        &self.ast_tree
    }
}