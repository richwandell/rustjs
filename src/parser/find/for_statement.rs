use crate::lexer::js_token::Tok;
use crate::parser::parser::SyntaxError;
use crate::parser::find::matching::{find_matching_paren, find_matching_brace};

pub(crate) fn find_end_of_for(start: usize, tokens: &Vec<Tok>) -> Result<usize, SyntaxError> {
    let mut j = start + 1;

    if let Some(Tok::Lpar) = tokens.get(j) {
        let k = find_matching_paren(j, tokens);
        j = k;
        if let Some(Tok::Lbrace) = tokens.get(j + 1) {
            let k = find_matching_brace(j + 1, tokens);
            j = k;
        } else {
            return Err(SyntaxError::UnexpectedToken {tok: tokens.get(j).unwrap().clone()})
        }
    } else {
        return Err(SyntaxError::UnexpectedToken {tok: tokens.get(j).unwrap().clone()})
    }
    return Ok(j);
}