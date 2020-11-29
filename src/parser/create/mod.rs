use crate::lexer::js_token::Tok;

pub(crate) mod function;
pub(crate) mod expression;
pub(crate) mod for_statement;
pub(crate) mod block_statement;
pub(crate) mod array_expression;
pub(crate) mod if_statement;

pub(crate) fn comma_separate_tokens(mut tokens: Vec<Tok>) -> Vec<Vec<Tok>> {
    let mut stack = vec![];
    let mut current = vec![];
    let mut all = vec![];
    while tokens.len() > 0 {
        let tok = tokens.pop().unwrap();
        match tok {
            Tok::Lsqb | Tok::Lbrace | Tok::Lpar => {
                stack.push(".");
                current.push(tok);
            }
            Tok::Rsqb | Tok::Rbrace | Tok::Rpar => {
                stack.pop();
                current.push(tok);
            }
            Tok::Comma => {
                if stack.len() == 0 {
                    all.push(current.clone());
                    current = vec![];
                }
            }
            _ => {
                current.push(tok);
            }
        }
    }
    if current.len() > 0 {
        all.push(current)
    }
    return all;
}