use crate::lexer::js_token::Tok;
use crate::parser::symbols::{JSItem, Statement, Expression};
use crate::parser::combine::{combine_star, combine_bslash, combine_plus, combine_minus, combine_float, combine_dot, combine_name, combine_string, combine_call, combine_expression};

pub(crate) fn create_assignment_expression(mut tokens: Vec<Tok>) -> JSItem {
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

pub(crate) fn create_expression(mut tokens: Vec<Tok>) -> JSItem {
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