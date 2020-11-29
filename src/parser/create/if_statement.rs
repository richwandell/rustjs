use crate::lexer::js_token::Tok;
use crate::parser::symbols::{JSItem, Expression, Statement};
use crate::parser::parser::Parser;

pub(crate) fn create_if_statement(mut tokens: Vec<Tok>) -> JSItem {
    tokens.reverse();

    // remove first if
    tokens.pop();

    // remove lpar
    tokens.pop();

    let mut test_expression_tokens = vec![];
    let mut stack = vec!["("];
    while !tokens.is_empty() {
        let tok = tokens.pop().unwrap();
        match tok {
            Tok::Lpar => {
                stack.push("(");
                test_expression_tokens.push(tok);
            }
            Tok::Rpar => {
                stack.pop();
                if stack.is_empty() {
                    break;
                } else {
                    test_expression_tokens.push(tok);
                }
            }
            _ => {
                test_expression_tokens.push(tok);
            }
        }
    }

    let mut consequent_tokens = vec![];
    if tokens.get(tokens.len() - 1).unwrap().eq(&Tok::Lbrace) {
        tokens.pop();
        let mut stack = vec!["{"];
        while !tokens.is_empty() {
            let tok = tokens.pop().unwrap();
            match tok {
                Tok::Lbrace => {
                    stack.push("{");
                    consequent_tokens.push(tok);
                }
                Tok::Rbrace => {
                    stack.pop();
                    if stack.is_empty() {
                        break;
                    } else {
                        consequent_tokens.push(tok);
                    }
                }
                _ => {
                    consequent_tokens.push(tok);
                }
            }
        }
    } else {
        tokens.pop();
        while !tokens.is_empty() {
            let tok = tokens.pop().unwrap();
            match tok {
                Tok::EndOfLine => {
                    break;
                }
                _ => {
                    consequent_tokens.push(tok);
                }
            }
        }
    }

    let mut alternate_tokens = vec![];
    if !tokens.is_empty() {
        if tokens.get(tokens.len() - 1).unwrap().eq(&Tok::Else) {
            tokens.pop();
            while !tokens.is_empty() {
                alternate_tokens.push(tokens.pop().unwrap());
            }
        }
    }

    let mut parser = Parser::new();
    let mut test_expression = parser.parse(test_expression_tokens);
    let mut consequent_expression = parser.parse(consequent_tokens);
    let mut alternate_expression = vec![JSItem::Ex { expression: Box::from(Expression::None)}];
    if alternate_tokens.len() > 0 {
        if alternate_tokens.get(0).unwrap().eq(&Tok::Lbrace) {
            alternate_tokens.remove(0);
            alternate_tokens.pop();
        }
        alternate_expression = parser.parse(alternate_tokens);
    }


    JSItem::St {
        statement: Box::from(Statement::If {
            test: test_expression.pop().unwrap(),
            consequent: consequent_expression,
            alternate: alternate_expression.pop().unwrap()
        })
    }
}