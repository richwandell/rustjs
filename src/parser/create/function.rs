use crate::lexer::js_token::Tok;
use crate::parser::symbols::{JSItem, Statement};
use crate::parser::parser::Parser;

pub(crate) fn create_arrow_function(mut tokens: Vec<Tok>) -> JSItem {
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
                if stack.is_empty() {
                    break;
                } else {
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
                if stack.is_empty() {
                    break;
                } else {
                    function_body.push(Tok::Rpar);
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

pub(crate) fn create_function_assignment(mut tokens: Vec<Tok>) -> JSItem {
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

pub(crate) fn create_function(mut tokens: Vec<Tok>) -> JSItem {
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