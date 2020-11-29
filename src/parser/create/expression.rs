use crate::lexer::js_token::Tok;
use crate::parser::symbols::{JSItem, Statement, Expression, AssignOp};
use crate::parser::combine::{combine_star, combine_bslash, combine_plus, combine_minus, combine_float, combine_dot, combine_name, combine_string, combine_call, combine_expression, combine_less, combine_array, combine_greater};
use crate::parser::parser::{Parser, SyntaxError};
use crate::parser::create::comma_separate_tokens;
use crate::parser::create::block_statement::create_object_expression;
use crate::parser::create::array_expression::create_array_expression;

pub(crate) fn create_assignment_expression(mut tokens: Vec<Tok>) -> Result<JSItem, SyntaxError> {
    tokens.reverse();
    let mut left = vec![];
    let mut right = vec![];

    let mut found_equal = false;
    while !tokens.is_empty() {
        let tok = tokens.pop().unwrap();
        if !found_equal && tok.eq(&Tok::Equal) {
            found_equal = true;
        } else if !found_equal {
            left.push(tok);
        } else {
            right.push(tok);
        }
    }

    if !found_equal {
        return Err(SyntaxError::UnexpectedToken {tok: left.pop().unwrap()})
    }

    let mut assign_op = AssignOp::None;
    if left.get(0).unwrap().eq(&Tok::Let) {
        left.remove(0);
        assign_op = AssignOp::Let;
    } else if left.get(0).unwrap().eq(&Tok::Const) {
        left.remove(0);
        assign_op = AssignOp::Const;
    } else if left.get(0).unwrap().eq(&Tok::Var) {
        left.remove(0);
        assign_op = AssignOp::Var;
    }

    if let JSItem::Ex {expression: left_expression} = create_expression(left) {
        let mut left ;
        if let Expression::Identifier {name} = *left_expression {
            left = Box::from(Expression::Literal {value: name});
        } else {
            left = left_expression;
        }

        loop {
            //get rid of EOL if it exists, we don't need it at this point.
            if let Tok::EndOfLine = right.get(right.len() - 1).unwrap() {
                right.pop();
            }else if let Tok::Semi = right.get(right.len() - 1).unwrap() {
                right.pop();
            } else {
                break;
            }
        }

        if right.get(0).unwrap().eq(&Tok::Lbrace) && right.get(right.len() - 1).unwrap().eq(&Tok::Rbrace) {
            if let Ok(item) = create_object_expression(right) {
                if let JSItem::Object { mutable, properties } = item {
                    return Ok(JSItem::St {
                        statement: Box::new(Statement::AssignmentExpression {
                            operator: assign_op,
                            left: JSItem::Ex { expression: left },
                            right: JSItem::Object { mutable, properties }
                        })
                    })
                }
            }
        } else if right.get(0).unwrap().eq(&Tok::Lsqb) && right.get(right.len() - 1).unwrap().eq(&Tok::Rsqb) {
            if let Ok(item) = create_array_expression(right) {
                if let JSItem::Ex { expression: right_expression } = item {
                    return Ok(JSItem::St {
                        statement: Box::new(Statement::AssignmentExpression {
                            operator: assign_op,
                            left: JSItem::Ex { expression: left },
                            right: JSItem::Ex {expression: right_expression}
                        })
                    })
                }
            }
        } else {
            if let JSItem::Ex { expression: right_expression } = create_expression(right) {
                return Ok(JSItem::St {
                    statement: Box::new(Statement::AssignmentExpression {
                        operator: assign_op,
                        left: JSItem::Ex { expression: left },
                        right: JSItem::Ex { expression: right_expression }
                    })
                })
            }
        }
    }

    return Ok(JSItem::Ex {expression: Box::new(Expression::None)})
}

fn parse_parameters(mut tokens: Vec<Tok>) -> Vec<JSItem> {
    let mut all_params = comma_separate_tokens(tokens);
    let mut exp_params = vec![];

    for p in all_params {
        let mut parser = Parser::new();
        let mut out = parser.parse(p);
        exp_params.push(out.pop().unwrap());
    }

    exp_params
}

pub(crate) fn create_expression(mut tokens: Vec<Tok>) -> JSItem {
    let mut parens_content = vec![];
    let mut expression_stack = vec![];
    while tokens.len() > 0 {
        let token = tokens.pop().unwrap();
        match token {
            Tok::PlusPlus => {
                let exp = Expression::UpdateExpression {
                    expression: Box::new(Expression::None),
                };
                expression_stack.push(exp);
            }
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
            Tok::Less => {
                let ex = expression_stack.pop().unwrap_or(Expression::None);
                let exp = combine_less(ex);
                expression_stack.push(exp);
            }
            Tok::Greater => {
                let ex = expression_stack.pop().unwrap_or(Expression::None);
                let exp = combine_greater(ex);
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
            Tok::Rsqb => {
                let mut sqb_content = vec![];
                let mut sqb_stack = vec![];
                sqb_stack.push("]");
                while tokens.len() > 0 {
                    let token = tokens.pop().unwrap();
                    match token {
                        Tok::Rsqb => {
                            sqb_stack.push("]");
                            sqb_content.push(token);
                        }
                        Tok::Lsqb => {
                            sqb_stack.pop();
                            if sqb_stack.is_empty() {
                                break;
                            } else {
                                sqb_content.push(token);
                            }
                        }
                        _ => {
                            sqb_content.push(token);
                        }
                    }
                }
                let elements = parse_parameters(sqb_content);
                let ex = expression_stack.pop().unwrap_or(Expression::None);
                let exp = combine_array(ex, elements);
                expression_stack.push(exp);
            }
            Tok::Rpar => {
                let mut parens_content = vec![];
                let mut parens_stack = vec![];
                parens_stack.push(")");
                while tokens.len() > 0 {
                    let token = tokens.pop().unwrap();
                    match token {
                        Tok::Rpar => {
                            parens_stack.push(")");
                            parens_content.push(token);
                        }
                        Tok::Lpar => {
                            parens_stack.pop();
                            if parens_stack.is_empty() {
                                break;
                            } else {
                                parens_content.push(token);
                            }
                        }
                        _ => {
                            parens_content.push(token);
                        }
                    }
                }
                let ex = expression_stack.pop().unwrap_or(Expression::None);
                if !tokens.is_empty() {
                    let next = tokens.pop().unwrap();
                    match next {
                        Tok::Name { name } => {
                            tokens.push(Tok::Name { name });
                            let parameters = parse_parameters(parens_content);
                            let exp = combine_call(ex, parameters);
                            expression_stack.push(exp);
                        }
                        _ => {
                            parens_content.reverse();
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
                    parens_content.reverse();
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

