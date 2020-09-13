use crate::lexer::js_token::Tok;
use crate::parser::symbols::{JSItem, Statement};
use crate::parser::parser::Parser;

pub(crate) fn create_for_statement(mut tokens: Vec<Tok>) -> JSItem {
    tokens.reverse();

    //remove for
    tokens.pop();

    //remove Lpar
    tokens.pop();

    let mut initialization_tokens = vec![];
    while !tokens.is_empty() {
        let tok = tokens.pop().unwrap();
        match tok {
            Tok::Semi => {
                break;
            }
            _ => {
                initialization_tokens.push(tok);
            }
        }
    }

    let mut condition_tokens = vec![];
    while !tokens.is_empty() {
        let tok = tokens.pop().unwrap();
        match tok {
            Tok::Semi => {
                break;
            }
            _ => {
                condition_tokens.push(tok);
            }
        }
    }

    let mut final_expression_tokens = vec![];
    let mut stack = vec!["("];
    while !tokens.is_empty() {
        let tok = tokens.pop().unwrap();
        match tok {
            Tok::Lpar => {
                stack.push("(");
                final_expression_tokens.push(tok);
            }
            Tok::Rpar => {
                stack.pop();
                if stack.is_empty() {
                    break;
                } else {
                    final_expression_tokens.push(tok);
                }
            }
            _ => {
                final_expression_tokens.push(tok);
            }
        }
    }

    let mut body_expression_tokens = vec![];
    stack.push("{");
    //remove lbrace
    tokens.pop();
    while !tokens.is_empty() {
        let tok = tokens.pop().unwrap();
        match tok {
            Tok::Lbrace => {
                stack.push("{");
                body_expression_tokens.push(tok);
            }
            Tok::Rbrace => {
                stack.pop();
                if stack.is_empty() {
                    break;
                } else {
                    body_expression_tokens.push(tok);
                }
            }
            _ => {
                body_expression_tokens.push(tok);
            }
        }
    }

    let mut parser = Parser::new();
    let mut initialization_expression = parser.parse(initialization_tokens);

    parser = Parser::new();
    let mut condition_expression = parser.parse(condition_tokens);

    parser = Parser::new();
    let mut final_expression = parser.parse(final_expression_tokens);

    parser = Parser::new();
    let body_expression = parser.parse(body_expression_tokens);

    return JSItem::St {
        statement: Box::new(Statement::ForStatement {
            init: initialization_expression.pop().unwrap(),
            test: condition_expression.pop().unwrap(),
            update: final_expression.pop().unwrap(),
            body: body_expression
        })
    };
}