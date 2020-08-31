use crate::lexer::lexer::Lexer;

use crate::lexer::js_token::Tok;
use crate::parser::symbols::{Operator, Expression, Statement, JSItem};
use crate::parser::symbols::Expression::{Identifier};

pub struct Parser {
    pub(crate) lexer: Lexer,
    pub ast_tree: Vec<Expression>,
}

impl Parser {
    fn find_end_of_assignment(start: usize, tokens: &Vec<Tok>) -> usize {
        let mut j = start + 1;

        let mut current_type = "assignment";
        while j < tokens.len() - 1 {
            let token = tokens.get(j as usize).unwrap();

            if current_type == "assignment" {
                match token {
                    Tok::Name { name: _ } => {
                        j += 1;
                        current_type = "name";
                    }
                    _ => {
                        return j;
                    }
                }
            } else if current_type == "name" {
                match token {
                    Tok::Equal => {
                        j += 1;
                        current_type = "equal";
                    }
                    _ => {
                        return j;
                    }
                }
            } else if current_type == "equal" {
                match token {
                    Tok::Name { name: _ } => {
                        let k = Parser::find_end_of_expression(j, tokens, "name");
                        j = k + 1;
                        current_type = "expression"
                    }
                    _ => {
                        return j;
                    }
                }
            } else if current_type == "expression" {
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

    fn find_end_of_expression(start: usize, tokens: &Vec<Tok>, start_type: &str) -> usize {
        let mut j = start + 1;

        let mut prev_type = start_type;
        while j < tokens.len() - 1 {
            let token = tokens.get(j as usize).unwrap();
            if prev_type == "float" {
                match token {
                    Tok::Minus => {
                        prev_type = "minus";
                        j += 1;
                    }
                    Tok::MinusEqual => {
                        prev_type = "minus_equal";
                        j += 1;
                    }
                    Tok::Plus => {
                        prev_type = "plus";
                        j += 1;
                    }
                    Tok::PlusEqual => {
                        prev_type = "plus_equal";
                        j += 1;
                    }
                    Tok::Bslash => {
                        prev_type = "bslash";
                        j += 1;
                    }
                    Tok::BslashEqual => {
                        prev_type = "bslash_equal";
                        j += 1;
                    }
                    Tok::Star => {
                        prev_type = "star";
                        j += 1;
                    }
                    Tok::StarEqual => {
                        prev_type = "star_equal";
                        j += 1;
                    }
                    Tok::Less => {
                        prev_type = "less";
                        j += 1;
                    }
                    Tok::LessEqual => {
                        prev_type = "less_equal";
                        j += 1;
                    }
                    Tok::Greater => {
                        prev_type = "greater";
                        j += 1;
                    }
                    Tok::GreaterEqual => {
                        prev_type = "greater_equal";
                        j += 1;
                    }
                    Tok::LeftShift => {
                        prev_type = "left_shift";
                        j += 1;
                    }
                    Tok::LeftShiftEqual => {
                        prev_type = "left_shift_equal";
                        j += 1;
                    }
                    Tok::RightShift => {
                        prev_type = "right_shift";
                        j += 1;
                    }
                    Tok::RightShiftEqual => {
                        prev_type = "right_shift_equal";
                        j += 1;
                    }
                    Tok::RightShiftUnsigned => {
                        prev_type = "right_shift_unsigned";
                        j += 1;
                    }
                    Tok::RightShiftUnsignedEqual => {
                        prev_type = "right_shift_unsigned_equal";
                        j += 1;
                    }
                    Tok::Rpar => {
                        prev_type = "rpar";
                        j += 1;
                    }
                    Tok::Rsqb => {
                        prev_type = "rsqb";
                        j += 1;
                    }
                    Tok::Comma => {
                        prev_type = "comma";
                        j += 1;
                    }
                    Tok::Rbrace => {
                        prev_type = "rbrace";
                        j += 1;
                    }
                    _ => {
                        return j;
                    }
                }
            } else if prev_type == "star" {
                match token {
                    Tok::Float {value: _} => {
                        prev_type = "float";
                        j += 1;
                    }
                    Tok::Name { name: _ } => {
                        prev_type = "name";
                        j += 1;
                    }
                    Tok::Lpar => {
                        let k = Parser::find_matching_paren(j - 1, tokens);
                        j = k + 1;
                        prev_type = "rpar";
                    }
                    _ => {
                        return j;
                    }
                }
            } else if prev_type == "bslash" {
                match token {
                    Tok::Name { name: _ } => {
                        prev_type = "name";
                        j += 1;
                    }
                    Tok::Float { value: _ } => {
                        prev_type = "float";
                        j += 1;
                    }
                    _ => {
                        return j;
                    }
                }
            } else if prev_type == "plus" {
                match token {
                    Tok::Float { value: _ } => {
                        prev_type = "float";
                        j += 1;
                    }
                    Tok::Name { name: _ } => {
                        prev_type = "name";
                        j += 1;
                    }
                    Tok::Lpar => {
                        prev_type = "lpar";
                        j += 1;
                    }
                    _ => {
                        return j;
                    }
                }
            } else if prev_type == "name" {
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
                    Tok::Name { name: _ } => {
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
                        j = k + 1;
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
                    Tok::Plus => {
                        prev_type = "plus";
                        j += 1;
                    }
                    Tok::Bslash => {
                        prev_type = "bslash";
                        j += 1;
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
                Tok::Float { value: _ } => {
                    let j = Parser::find_end_of_expression(i, &tokens, "float");
                    let t = tokens[i..=j].to_vec();
                    let ex = self.create_expression(t);
                    js_items.push(ex);
                    i = j;
                }
                Tok::Let => {
                    //assignment
                    let j = Parser::find_end_of_assignment(i, &tokens);
                    // let t = tokens[i + 3..=j].to_vec();
                    // let mut p = Parser::new();
                    // let out = p.parse(t);
                    i = j;
                }
                Tok::Name { name: _ } => {
                    //expression
                    let j = Parser::find_end_of_expression(i, &tokens, "name");
                    let t = tokens[i..=j].to_vec();
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
                    let t = tokens[i..=j].to_vec();
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
                                property,
                            }),
                            arguments,
                        };
                    }
                    _ => {}
                }
            }
            Expression::MemberExpression { object, property } => {
                match *object {
                    Expression::Identifier { name } => {
                        let new_object = Parser::combine_dot(Expression::Identifier { name }, tok);
                        return Expression::MemberExpression {
                            object: Box::from(new_object),
                            property,
                        };
                    }
                    Expression::None => {}
                    Expression::MemberExpression { object, property } => {
                        let new_object = Parser::combine_dot(*object, tok);
                        let new_expression = Expression::MemberExpression {
                            object: Box::new(new_object),
                            property,
                        };
                        return new_expression;
                    }
                    _ => {}
                }
            }
            Expression::Identifier { name } => {
                return Expression::MemberExpression {
                    object: Box::new(Expression::None),
                    property: Box::new(Identifier { name }),
                };
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
                            arguments,
                        };
                    }
                    Expression::MemberExpression { object, property } => {
                        let new_object = Parser::combine_name(*object, name);

                        return Expression::CallExpression {
                            callee: Box::new(Expression::MemberExpression {
                                object: Box::from(new_object),
                                property,
                            }),
                            arguments,
                        };
                    }
                    _ => {}
                }
            }
            Expression::Identifier { name } => {
                return Expression::MemberExpression {
                    object: Box::new(Expression::None),
                    property: Box::new(Identifier { name }),
                };
            }
            Expression::None => {
                return Expression::Identifier { name };
            }
            Expression::MemberExpression { object, property } => {
                let outer_property = property;
                match *object {
                    Expression::None => {
                        return Expression::MemberExpression {
                            object: Box::from(Parser::combine_name(Expression::None, name)),
                            property: outer_property,
                        };
                    }
                    Expression::MemberExpression { object, property } => {
                        return Expression::MemberExpression {
                            object: Box::from(Expression::MemberExpression {
                                object: Box::new(Parser::combine_name(*object, name)),
                                property,
                            }),
                            property: outer_property,
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
                    arguments: params,
                };
            }
            Expression::CallExpression { callee, arguments } => {
                match *callee {
                    Expression::None => {
                        return Expression::CallExpression {
                            callee: Box::new(Expression::CallExpression {
                                callee: Box::new(Expression::None),
                                arguments: params,
                            }),
                            arguments,
                        };
                    }
                    Expression::MemberExpression { object, property } => {
                        let new_object = Parser::combine_call(*object, params);

                        return Expression::CallExpression {
                            callee: Box::new(Expression::MemberExpression {
                                object: Box::from(new_object),
                                property,
                            }),
                            arguments,
                        };
                    }
                    _ => {}
                }
            }
            Expression::Identifier { name } => {
                return Expression::MemberExpression {
                    object: Box::new(Expression::None),
                    property: Box::new(Identifier { name }),
                };
            }
            Expression::MemberExpression { object, property } => {
                let outer_property = property;
                match *object {
                    Expression::None => {
                        return Expression::MemberExpression {
                            object: Box::from(Parser::combine_call(Expression::None, params)),
                            property: outer_property,
                        };
                    }
                    Expression::MemberExpression { object, property } => {
                        return Expression::MemberExpression {
                            object: Box::from(Expression::MemberExpression {
                                object: Box::new(Parser::combine_call(*object, params)),
                                property,
                            }),
                            property: outer_property,
                        };
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        return Expression::None;
    }

    fn combine_float(last_exp: Expression, f_value: f64) -> Expression {
        match last_exp {
            Expression::Binop { a, op, b } => {
                match op {
                    Operator::Add => {
                        let new_a = Parser::combine_float(*a, f_value);
                        return Expression::Binop {
                            a: Box::from(new_a),
                            op,
                            b,
                        };
                    }
                    Operator::Div | Operator::Mult => {
                        let new_a = Parser::combine_float(*a, f_value);
                        return Expression::Binop {
                            a: Box::from(new_a),
                            op,
                            b,
                        };
                    }
                    _ => {}
                }
            }
            Expression::None => {
                return Expression::Number { value: f_value };
            }
            _ => {}
        }
        return Expression::None;
    }

    fn combine_plus(last_exp: Expression) -> Expression {
        match last_exp {
            Expression::Number { value } => {
                return Expression::Binop {
                    a: Box::new(Expression::None),
                    op: Operator::Add,
                    b: Box::new(Expression::Number { value }),
                };
            }
            Expression::Binop { a, op, b } => {
                match op {
                    Operator::Add => {
                        let new_exp = Parser::combine_plus(*a);
                        return Expression::Binop {
                            a: Box::from(new_exp),
                            op,
                            b,
                        };
                    }
                    Operator::Div => {
                        return Expression::Binop {
                            a: Box::from(Expression::None),
                            op: Operator::Add,
                            b: Box::from(Expression::Binop {
                                a,
                                op,
                                b,
                            }),
                        };
                    }
                    _ => {}
                }
            }
            Expression::SubExpression { expression } => {
                return Expression::Binop {
                    a: Box::new(Expression::None),
                    op: Operator::Add,
                    b: Box::new(Expression::SubExpression { expression }),
                };
            }
            _ => {}
        }
        return Expression::None;
    }

    fn combine_bslash(last_exp: Expression) -> Expression {
        match last_exp {
            Expression::Number { value } => {
                return Expression::Binop {
                    a: Box::new(Expression::None),
                    op: Operator::Div,
                    b: Box::new(Expression::Number { value }),
                };
            }
            Expression::Binop { a, op, b } => {
                let new_exp = Parser::combine_bslash(*a);
                return Expression::Binop {
                    a: Box::from(new_exp),
                    op,
                    b,
                };
            }
            _ => {}
        }
        return Expression::None;
    }

    fn combine_star(last_exp: Expression) -> Expression {
        match last_exp {
            Expression::Number { value } => {
                return Expression::Binop {
                    a: Box::new(Expression::None),
                    op: Operator::Mult,
                    b: Box::new(Expression::Number { value }),
                };
            }
            Expression::Binop { a, op, b } => {
                let new_exp = Parser::combine_star(*a);
                return Expression::Binop {
                    a: Box::from(new_exp),
                    op,
                    b,
                };
            }
            _ => {}
        }
        return Expression::None;
    }

    fn combine_expression(last_exp: Expression, next_expression: Expression) -> Expression {
        match last_exp {
            Expression::Binop { a: _, op, b } => {
                return Expression::Binop {
                    a: Box::new(Expression::SubExpression {
                        expression: Box::from(next_expression)
                    }),
                    op,
                    b,
                };
            }
            _ => {}
        }
        return Expression::None;
    }

    fn create_expression(&mut self, mut tokens: Vec<Tok>) -> JSItem {
        let mut parens_content = vec![];
        let mut expression_stack = vec![];
        while tokens.len() > 0 {
            let token = tokens.pop().unwrap();
            match token {
                Tok::Star => {
                    let ex = expression_stack.pop().unwrap();
                    let exp = Parser::combine_star(ex);
                    expression_stack.push(exp);
                }
                Tok::Bslash => {
                    let ex = expression_stack.pop().unwrap();
                    let exp = Parser::combine_bslash(ex);
                    expression_stack.push(exp);
                }
                Tok::Plus => {
                    let ex = expression_stack.pop().unwrap_or(Expression::None);
                    let exp = Parser::combine_plus(ex);
                    expression_stack.push(exp);
                }
                Tok::Float { value } => {
                    let ex = expression_stack.pop().unwrap_or(Expression::None);
                    let exp = Parser::combine_float(ex, value);
                    expression_stack.push(exp);
                }
                Tok::Dot => {
                    let ex = expression_stack.pop().unwrap();
                    let exp = Parser::combine_dot(ex, token);
                    expression_stack.push(exp);
                }
                Tok::Name { name } => {
                    let ex = expression_stack.pop().unwrap_or(Expression::None);
                    let exp = Parser::combine_name(ex, name);
                    expression_stack.push(exp);
                }
                Tok::Semi | Tok::EndOfLine => {}
                Tok::Comma => {}
                Tok::Rpar => {
                    let mut parens_content = vec![];
                    let mut parens_stack = vec![];
                    parens_stack.push(")");
                    while tokens.len() > 0 {
                        let token = tokens.pop().unwrap();
                        match token {
                            Tok::Rpar => {
                                parens_stack.push(")")
                            }
                            Tok::Lpar => {
                                parens_stack.pop();
                                if parens_stack.is_empty() {
                                    break;
                                }
                            }
                            _ => {
                                parens_content.push(token);
                            }
                        }
                    }

                    parens_content.reverse();

                    let ex = expression_stack.pop().unwrap_or(Expression::None);

                    if !tokens.is_empty() {
                        let next = tokens.pop().unwrap();
                        match next {
                            Tok::Name { name } => {
                                tokens.push(Tok::Name { name });
                                let exp = Parser::combine_call(ex, parens_content.clone());
                                expression_stack.push(exp);
                            }
                            _ => {
                                tokens.push(next);
                                let item = self.create_expression(parens_content);
                                match item {
                                    JSItem::Ex { expression } => {
                                        let exp = Parser::combine_expression(ex, *expression);
                                        expression_stack.push(exp);
                                    }
                                    JSItem::St { statement: _ } => {}
                                }
                            }
                        }
                    } else {
                        let item = self.create_expression(parens_content);
                        match item {
                            JSItem::Ex { expression } => {
                                let exp = Parser::combine_expression(ex, *expression);
                                expression_stack.push(exp);
                            }
                            JSItem::St { statement: _ } => {}
                        }
                    }
                }
                _ => {
                    parens_content.push(token);
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
            ast_tree: vec![],
        }
    }

    pub fn get_ast(&mut self) -> &Vec<Expression> {
        &self.ast_tree
    }
}