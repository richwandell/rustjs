use crate::lexer::js_token::Tok;
use crate::parser::parser::{AssignmentType, SyntaxError};
use crate::parser::find::function::{find_arrow_function, find_function_assignment};
use crate::parser::find::matching::{find_matching_paren, find_matching_brace};
use crate::parser::find::expression::find_end_of_expression;

pub(crate) fn find_end_of_assignment(start: usize, tokens: &Vec<Tok>) -> Result<AssignmentType, SyntaxError> {
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
                Tok::Lsqb => {
                    let k = find_end_of_expression(j + 2, tokens, "lsqb");
                    current_type = "expression";
                    j = k;
                }
                Tok::Lbrace => {
                    let k = find_matching_brace(j + 2, tokens);
                    current_type = "rbrace";
                    j = k + 1;
                    if j < tokens.len() - 1 {
                        if let Tok::Semi = tokens.get(j).unwrap() {
                            return Ok(AssignmentType::ObjectExpression {end: j - 1});
                        }
                    }
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