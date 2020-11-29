use crate::lexer::js_token::Tok;


pub(crate) fn find_matching_paren(start: usize, tokens: &Vec<Tok>) -> usize {
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

pub(crate) fn find_matching_brace(start: usize, tokens: &Vec<Tok>) -> usize {
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

pub(crate) fn find_matching_sqb(start: usize, tokens: &Vec<Tok>) -> usize {
    let mut j = start;
    let mut lsqb = 0;

    while j < tokens.len() {
        let token = tokens.get(j as usize).unwrap();
        if token.eq(&Tok::Lsqb) {
            lsqb += 1;
        } else if token.eq(&Tok::Rsqb) {
            lsqb -= 1;
            if lsqb == 0 {
                break;
            }
        }
        j += 1;
    }
    return j;
}

pub(crate) fn find_end_of_line_or_lbrace(start: usize, tokens: &Vec<Tok>) -> usize {
    let mut j = start;
    while j < tokens.len() {
        let token = tokens.get(j as usize).unwrap();
        if token.eq(&Tok::EndOfLine) {
            return j;
        }
        if token.eq(&Tok::Lbrace) {
            return j;
        }
        j += 1;
    }
    return j;
}