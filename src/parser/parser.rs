use crate::lexer::js_token::Tok;
use crate::parser::symbols::{Expression, Statement, JSItem};
use crate::parser::combine::{combine_star, combine_bslash, combine_plus, combine_minus,
                             combine_float, combine_dot, combine_name, combine_string,
                             combine_call, combine_expression};

pub(crate) struct Parser {
    pub ast_tree: Vec<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum SyntaxError {
    UnexpectedToken {
        tok: Tok
    },
    UnexpectedIdentifier {
        name: String
    },
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum AssignmentType {
    ArrowFunction {
        end: usize
    },
    FunctionAssignment {
        end: usize
    },
    Unknown {
        end: usize
    },
    Expression {
        end: usize
    },
}

fn find_arrow_function(start: usize, tokens: &Vec<Tok>) -> usize {
    return match tokens.get(start).unwrap() {
        Tok::Let | Tok::Const => {
            match tokens.get(start + 1).unwrap() {
                Tok::Name { name: _ } => {
                    match tokens.get(start + 2).unwrap() {
                        Tok::Equal => {
                            match tokens.get(start + 3).unwrap() {
                                Tok::Lpar => {
                                    let j = find_matching_paren(start + 3, tokens);
                                    match tokens.get(j + 1).unwrap() {
                                        Tok::RdoubleArrow => {
                                            match tokens.get(j + 2).unwrap() {
                                                Tok::Lbrace => {
                                                    let j = find_matching_brace(j + 2, tokens);
                                                    j
                                                }
                                                _ => start
                                            }
                                        }
                                        _ => start
                                    }
                                }
                                _ => start
                            }
                        }
                        _ => start
                    }
                }
                _ => start
            }
        }
        _ => start
    };
}

fn find_function_assignment(start: usize, tokens: &Vec<Tok>) -> usize {
    return match tokens.get(start).unwrap() {
        Tok::Let | Tok::Const => {
            match tokens.get(start + 1).unwrap() {
                Tok::Name { name: _ } => {
                    match tokens.get(start + 2).unwrap() {
                        Tok::Equal => {
                            match tokens.get(start + 3).unwrap() {
                                Tok::Function => {
                                    match tokens.get(start + 4).unwrap() {
                                        Tok::Lpar => {
                                            let j = find_matching_paren(start + 4, tokens);
                                            match tokens.get(j + 1).unwrap() {
                                                Tok::Lbrace => {
                                                    let j = find_matching_brace(j + 1, tokens);
                                                    j
                                                }
                                                _ => start
                                            }
                                        }
                                        _ => start
                                    }
                                }
                                _ => start
                            }
                        }
                        _ => start
                    }
                }
                _ => start
            }
        }
        _ => start
    };
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

fn find_matching_brace(start: usize, tokens: &Vec<Tok>) -> usize {
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
                Tok::Float { value: _ } => {
                    prev_type = "float";
                    j += 1;
                }
                Tok::Name { name: _ } => {
                    prev_type = "name";
                    j += 1;
                }
                Tok::Lpar => {
                    let k = find_matching_paren(j - 1, tokens);
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
        } else if prev_type == "minus" {
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
            let k = find_matching_paren(j - 1, tokens);
            j = k + 1;
            prev_type = "rpar";
        } else if prev_type == "rpar" {
            match token {
                Tok::Lpar => {
                    let k = find_matching_paren(j - 1, tokens);
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
                Tok::Minus => {
                    prev_type = "minus";
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
        } else if prev_type == "string" {
            match token {
                Tok::Semi | Tok::EndOfLine => {
                    return j;
                }
                _ => {
                    j += 1;
                }
            }
        } else {
            j += 1;
        }
    }
    if j == tokens.len() {
        return j - 1;
    }
    return j;
}

fn find_end_of_assignment(start: usize, tokens: &Vec<Tok>) -> Result<AssignmentType, SyntaxError> {
    let mut current_type = "assignment";
    let mut j = start + 1;

    let arrow_function = find_arrow_function(start, tokens);
    if arrow_function > start {
        j = arrow_function + 1;
        current_type = "arrow_function";
    } else {
        let function_assignment = find_function_assignment(start, tokens);
        if function_assignment > start {
            j = function_assignment + 1;
            current_type = "function_assignment";
        } else {
            let token = tokens.get(j + 2 as usize).unwrap();
            match token {
                Tok::Float { value: _ } => {
                    let k = find_end_of_expression(j + 2, tokens, "float");
                    current_type = "expression";
                    j = k;
                }
                Tok::String { value: _ } => {
                    let k = find_end_of_expression(j + 2, tokens, "string");
                    current_type = "expression";
                    j = k;
                }
                Tok::Name { name: _ } => {
                    let k = find_end_of_expression(j + 2, tokens, "name");
                    current_type = "expression";
                    j = k;
                }
                Tok::Lpar => {
                    let k = find_end_of_expression(j + 2, tokens, "lpar");
                    current_type = "expression";
                    j = k;
                }
                _ => {}
            }
        }
    }

    if j == tokens.len() - 1 {
        if current_type == "expression" {
            return Ok(AssignmentType::Expression { end: j });
        } else if current_type == "function_assignment" {
            return Ok(AssignmentType::FunctionAssignment {end: j});
        } else if current_type == "arrow_function" {
            return Ok(AssignmentType::ArrowFunction {end: j});
        }
    }

    while j <= tokens.len() - 1 {
        let token = tokens.get(j as usize).unwrap();

        if current_type == "assignment" {
            match token {
                Tok::Name { name: _ } => {
                    j += 1;
                    current_type = "name";
                }
                _ => {
                    return Err(SyntaxError::UnexpectedToken { tok: token.clone() });
                }
            }
        } else if current_type == "name" {
            match token {
                Tok::Equal => {
                    j += 1;
                    current_type = "equal";
                }
                _ => {
                    return Err(SyntaxError::UnexpectedToken { tok: token.clone() });
                }
            }
        } else if current_type == "equal" {
            match token {
                Tok::Name { name: _ } => {
                    let k = find_end_of_expression(j, tokens, "name");
                    j = k + 1;
                    current_type = "expression"
                }
                Tok::Lpar => {
                    let k = find_matching_paren(j, tokens);
                    j = k + 1;
                    current_type = "rpar";
                }
                Tok::String { value: _ } => {
                    j = j + 1;
                    current_type = "string";
                }
                _ => {
                    return Err(SyntaxError::UnexpectedToken { tok: token.clone() });
                }
            }
        } else if current_type == "string" {
            match token {
                Tok::Plus => {
                    j = j + 1;
                    current_type = "plus";
                }
                Tok::Minus => {
                    j = j + 1;
                    current_type = "minus";
                }
                Tok::Dot => {
                    j = j + 1;
                    current_type = "dot";
                }
                Tok::Bslash => {
                    j = j + 1;
                    current_type = "bslash";
                }
                Tok::Star => {
                    j = j + 1;
                    current_type = "star";
                }
                _ => {
                    return Err(SyntaxError::UnexpectedToken { tok: token.clone() });
                }
            }
        } else if current_type == "expression" {
            match token {
                Tok::Lpar => {
                    let k = find_matching_paren(j, tokens);
                    j = k + 1;
                }
                Tok::Semi | Tok::EndOfLine => {
                    return Ok(AssignmentType::Expression { end: j });
                }
                _ => {
                    return Err(SyntaxError::UnexpectedToken { tok: token.clone() });
                }
            }
        } else if current_type == "arrow_function" {
            match token {
                Tok::Semi | Tok::EndOfLine => {
                    return Ok(AssignmentType::ArrowFunction { end: j });
                }
                Tok::Name { name } => {
                    return Err(SyntaxError::UnexpectedIdentifier { name: String::from(name) });
                }
                _ => {
                    return Err(SyntaxError::UnexpectedToken { tok: token.clone() });
                }
            }
        } else if current_type == "function_assignment" {
            match token {
                Tok::Semi | Tok::EndOfLine => {
                    return Ok(AssignmentType::FunctionAssignment { end: j });
                }
                Tok::Name { name } => {
                    return Err(SyntaxError::UnexpectedIdentifier { name: String::from(name) });
                }
                _ => {
                    return Err(SyntaxError::UnexpectedToken { tok: token.clone() });
                }
            }
        }
    }
    if j == tokens.len() {
        return Ok(AssignmentType::Unknown { end: j - 1 });
    }
    return Ok(AssignmentType::Unknown { end: j });
}

fn create_arrow_function(mut tokens: Vec<Tok>) -> JSItem {
    tokens.reverse();
    let mutable = tokens.pop().unwrap();
    let mut function_name = "".to_string();
    match tokens.pop().unwrap() {
        Tok::Name { name } => {
            function_name = String::from(name);
        }
        _ => {}
    }
    //get rid of equal
    tokens.pop();


    let mut function_args = vec![];
    let mut stack = vec![];
    stack.push(tokens.pop().unwrap());
    while !stack.is_empty() {
        let tok = tokens.pop().unwrap();
        match tok {
            Tok::Rpar => {
                stack.pop();
                if !stack.is_empty() {
                    function_args.push(Tok::Rpar);
                }
            }
            Tok::Lpar => {
                stack.push(Tok::Lpar);
                function_args.push(Tok::Lpar);
            }
            _ => {
                function_args.push(tok)
            }
        }
    }

    //get rid of rdoublearrow
    tokens.pop();

    let mut function_body = vec![];
    stack.push(tokens.pop().unwrap());
    while !stack.is_empty() {
        let tok = tokens.pop().unwrap();
        match tok {
            Tok::Rbrace => {
                stack.pop();
                if !stack.is_empty() {
                    function_body.push(Tok::Rbrace);
                }
            }
            Tok::Lbrace => {
                stack.push(Tok::Lbrace);
                function_body.push(Tok::Lbrace);
            }
            _ => {
                function_body.push(tok)
            }
        }
    }

    let mut p = Parser::new();
    let out = p.parse(function_body);
    let statement = Box::new(Statement::AssignArrowFunction {
        mutable: mutable.eq(&Tok::Let),
        function: Box::new(Statement::FunctionDef {
            name: function_name,
            params: function_args,
            body: out,
        }),
    });
    let item = JSItem::St { statement };
    return item;
}

fn create_function_assignment(mut tokens: Vec<Tok>) -> JSItem {
    tokens.reverse();
    let mutable = tokens.pop().unwrap();
    let mut function_name = "".to_string();
    match tokens.pop().unwrap() {
        Tok::Name { name } => {
            function_name = String::from(name);
        }
        _ => {}
    }
    //get rid of equal
    tokens.pop();
    //get rid of function keyword
    tokens.pop();

    let mut function_args = vec![];
    let mut stack = vec![];
    stack.push(tokens.pop().unwrap());
    while !stack.is_empty() {
        let tok = tokens.pop().unwrap();
        match tok {
            Tok::Rpar => {
                stack.pop();
                if !stack.is_empty() {
                    function_args.push(Tok::Rpar);
                }
            }
            Tok::Lpar => {
                stack.push(Tok::Lpar);
                function_args.push(Tok::Lpar);
            }
            _ => {
                function_args.push(tok)
            }
        }
    }

    let mut function_body = vec![];
    stack.push(tokens.pop().unwrap());
    while !stack.is_empty() {
        let tok = tokens.pop().unwrap();
        match tok {
            Tok::Rbrace => {
                stack.pop();
                if !stack.is_empty() {
                    function_body.push(Tok::Rbrace);
                }
            }
            Tok::Lbrace => {
                stack.push(Tok::Lbrace);
                function_body.push(Tok::Lbrace);
            }
            _ => {
                function_body.push(tok)
            }
        }
    }

    let mut p = Parser::new();
    let out = p.parse(function_body);
    let statement = Box::new(Statement::AssignFunction {
        mutable: mutable.eq(&Tok::Let),
        function: Box::new(Statement::FunctionDef {
            name: function_name,
            params: function_args,
            body: out,
        }),
    });
    let item = JSItem::St { statement };
    return item;
}

fn create_assignment_expression(mut tokens: Vec<Tok>) -> JSItem {
    tokens.reverse();
    let mutable = tokens.pop().unwrap();
    let mut variable_name = "".to_string();
    match tokens.pop().unwrap() {
        Tok::Name { name } => {
            variable_name = String::from(name);
        }
        _ => {}
    }
    //get rid of equal
    tokens.pop();

    tokens.reverse();
    let exp = create_expression(tokens);
    match exp {
        JSItem::Ex {expression} => {
            return JSItem::St {
                statement: Box::new(Statement::AssignExpression {
                    mutable: mutable.eq(&Tok::Let),
                    name: variable_name,
                    value: expression
                })
            }
        }
        _ => {
            return JSItem::Ex {expression: Box::new(Expression::None)};
        }
    }
}

fn create_expression(mut tokens: Vec<Tok>) -> JSItem {
    let mut parens_content = vec![];
    let mut expression_stack = vec![];
    while tokens.len() > 0 {
        let token = tokens.pop().unwrap();
        match token {
            Tok::Star => {
                let ex = expression_stack.pop().unwrap();
                let exp = combine_star(ex);
                expression_stack.push(exp);
            }
            Tok::Bslash => {
                let ex = expression_stack.pop().unwrap();
                let exp = combine_bslash(ex);
                expression_stack.push(exp);
            }
            Tok::Plus => {
                let ex = expression_stack.pop().unwrap_or(Expression::None);
                let exp = combine_plus(ex);
                expression_stack.push(exp);
            }
            Tok::Minus => {
                let ex = expression_stack.pop().unwrap_or(Expression::None);
                let exp = combine_minus(ex);
                expression_stack.push(exp);
            }
            Tok::Float { value } => {
                let ex = expression_stack.pop().unwrap_or(Expression::None);
                let exp = combine_float(ex, value);
                expression_stack.push(exp);
            }
            Tok::Dot => {
                let ex = expression_stack.pop().unwrap();
                let exp = combine_dot(ex, token);
                expression_stack.push(exp);
            }
            Tok::Name { name } => {
                let ex = expression_stack.pop().unwrap_or(Expression::None);
                let exp = combine_name(ex, name);
                expression_stack.push(exp);
            }
            Tok::String { value } => {
                let ex = expression_stack.pop().unwrap_or(Expression::None);
                let exp = combine_string(ex, value);
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
                            let exp = combine_call(ex, parens_content.clone());
                            expression_stack.push(exp);
                        }
                        _ => {
                            tokens.push(next);
                            let item = create_expression(parens_content);
                            match item {
                                JSItem::Ex { expression } => {
                                    let exp = combine_expression(ex, *expression);
                                    expression_stack.push(exp);
                                }
                                JSItem::St { statement: _ } => {}
                                _ => {}
                            }
                        }
                    }
                } else {
                    let item = create_expression(parens_content);
                    match item {
                        JSItem::Ex { expression } => {
                            let exp = combine_expression(ex, *expression);
                            expression_stack.push(exp);
                        }
                        JSItem::St { statement: _ } => {}
                        _ => {}
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

fn create_function(mut tokens: Vec<Tok>) -> JSItem {
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

impl Parser {
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
                    let j = find_end_of_expression(i, &tokens, "float");
                    let t = tokens[i..=j].to_vec();
                    let ex = create_expression(t);
                    js_items.push(ex);
                    i = j;
                }
                Tok::Let | Tok::Const => {
                    //assignment
                    let assignment_type = find_end_of_assignment(i, &tokens).unwrap();
                    match assignment_type {
                        AssignmentType::Unknown { end: _ } => {}
                        AssignmentType::ArrowFunction { end } => {
                            let t = tokens[i..=end].to_vec();
                            let assign = create_arrow_function(t);
                            js_items.push(assign);
                            i = end;
                        }
                        AssignmentType::Expression { end } => {
                            let t = tokens[i..=end].to_vec();
                            let ex = create_assignment_expression(t);
                            js_items.push(ex);
                            i = end;
                        }
                        AssignmentType::FunctionAssignment { end } => {
                            let t = tokens[i..=end].to_vec();
                            let assign = create_function_assignment(t);
                            js_items.push(assign);
                            i = end;
                        }
                    }
                }
                Tok::Name { name: _ } => {
                    //expression
                    let j = find_end_of_expression(i, &tokens, "name");
                    let t = tokens[i..=j].to_vec();
                    let ex = create_expression(t);
                    js_items.push(ex);
                    i = j;
                }
                Tok::Lpar => {
                    let j = find_end_of_expression(i, &tokens, "lpar");
                    let t = tokens[i..=j].to_vec();
                    let ex = create_expression(t);
                    js_items.push(ex);
                    i = j;
                }
                Tok::Function => {
                    //function
                    let j = find_matching_brace(i + 1, &tokens);
                    let t = tokens[i..=j].to_vec();
                    let func = create_function(t);
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

    pub fn new() -> Parser {
        Parser {
            ast_tree: vec![],
        }
    }
}