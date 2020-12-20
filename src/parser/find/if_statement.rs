use crate::lexer::js_token::Tok;
use crate::parser::parser::SyntaxError;
use crate::parser::find::matching::{find_matching_paren, find_end_of_line_or_lbrace, find_matching_brace};


pub(crate) fn find_end_of_if(start: usize, tokens: &Vec<Tok>) -> Result<usize, SyntaxError> {
    let mut j = start + 1;

    if let Some(Tok::Lpar) = tokens.get(j) {
        let k = find_matching_paren(j, tokens);
        j = k;

        loop {

            let k = find_end_of_line_or_lbrace(j, tokens);
            j = k;

            if j == tokens.len() {
                j = tokens.len() - 1;
                break;
            }

            if j == tokens.len() - 1 {
                break;
            }


            match tokens.get(k).unwrap() {
                Tok::EndOfLine => {
                    match tokens.get(k + 1).unwrap() {
                        Tok::Else => {

                        }
                        _ => {
                            j = k;
                            let k = find_end_of_line_or_lbrace(j + 1, tokens);
                            j = k;
                            continue;
                        }
                    }
                }
                Tok::Lbrace => {
                    let k = find_matching_brace(j, tokens);
                    j = k;
                }
                _ => {
                    break;
                }
            }


            if j == tokens.len() {
                j = tokens.len() - 1;
                break;
            }

            if j == tokens.len() - 1 {
                break;
            }

            match tokens.get(j + 1).unwrap() {
                Tok::Else => {
                    j = j + 1;
                    continue;
                }
                _ => {
                    break;
                }
            }
        }
    } else {
        return Err(SyntaxError::UnexpectedToken {tok: tokens.get(j).unwrap().clone()});
    }

    return Ok(j);
}