use crate::lexer::js_token::Tok;
use crate::parser::find::matching::find_matching_paren;

pub(crate) fn find_end_of_expression(start: usize, tokens: &Vec<Tok>, start_type: &str) -> usize {
    let mut j = start + 1;

    let mut prev_type = start_type;
    while j < tokens.len() - 1 {
        let token = tokens.get(j as usize).unwrap();
        if prev_type == "float" {
            match token {
                Tok::Minus => {
                    prev_type = "minus";
                    j += 1;
                }
                Tok::MinusEqual => {
                    prev_type = "minus_equal";
                    j += 1;
                }
                Tok::Plus => {
                    prev_type = "plus";
                    j += 1;
                }
                Tok::PlusEqual => {
                    prev_type = "plus_equal";
                    j += 1;
                }
                Tok::Bslash => {
                    prev_type = "bslash";
                    j += 1;
                }
                Tok::BslashEqual => {
                    prev_type = "bslash_equal";
                    j += 1;
                }
                Tok::Star => {
                    prev_type = "star";
                    j += 1;
                }
                Tok::StarEqual => {
                    prev_type = "star_equal";
                    j += 1;
                }
                Tok::Less => {
                    prev_type = "less";
                    j += 1;
                }
                Tok::LessEqual => {
                    prev_type = "less_equal";
                    j += 1;
                }
                Tok::Greater => {
                    prev_type = "greater";
                    j += 1;
                }
                Tok::GreaterEqual => {
                    prev_type = "greater_equal";
                    j += 1;
                }
                Tok::LeftShift => {
                    prev_type = "left_shift";
                    j += 1;
                }
                Tok::LeftShiftEqual => {
                    prev_type = "left_shift_equal";
                    j += 1;
                }
                Tok::RightShift => {
                    prev_type = "right_shift";
                    j += 1;
                }
                Tok::RightShiftEqual => {
                    prev_type = "right_shift_equal";
                    j += 1;
                }
                Tok::RightShiftUnsigned => {
                    prev_type = "right_shift_unsigned";
                    j += 1;
                }
                Tok::RightShiftUnsignedEqual => {
                    prev_type = "right_shift_unsigned_equal";
                    j += 1;
                }
                Tok::Rpar => {
                    prev_type = "rpar";
                    j += 1;
                }
                Tok::Rsqb => {
                    prev_type = "rsqb";
                    j += 1;
                }
                Tok::Comma => {
                    prev_type = "comma";
                    j += 1;
                }
                Tok::Rbrace => {
                    prev_type = "rbrace";
                    j += 1;
                }
                _ => {
                    return j;
                }
            }
        }
        else if prev_type == "star" {
            match token {
                Tok::Float { value: _ } => {
                    prev_type = "float";
                    j += 1;
                }
                Tok::Name { name: _ } => {
                    prev_type = "name";
                    j += 1;
                }
                Tok::Lpar => {
                    let k = find_matching_paren(j - 1, tokens);
                    j = k + 1;
                    prev_type = "rpar";
                }
                _ => {
                    return j;
                }
            }
        }
        else if prev_type == "bslash" {
            match token {
                Tok::Name { name: _ } => {
                    prev_type = "name";
                    j += 1;
                }
                Tok::Float { value: _ } => {
                    prev_type = "float";
                    j += 1;
                }
                _ => {
                    return j;
                }
            }
        }
        else if prev_type == "plus" {
            match token {
                Tok::Float { value: _ } => {
                    prev_type = "float";
                    j += 1;
                }
                Tok::Name { name: _ } => {
                    prev_type = "name";
                    j += 1;
                }
                Tok::Lpar => {
                    prev_type = "lpar";
                    j += 1;
                }
                _ => {
                    return j;
                }
            }
        }
        else if prev_type == "plus_plus" {
            match token {
                Tok::Semi => {
                    return j;
                }
                _ => {
                    return j;
                }
            }
        }
        else if prev_type == "less" {
            match token {
                Tok::Float {value: _} =>{
                    prev_type = "float";
                    j += 1;
                }
                Tok::Name {name:_}=>{
                    prev_type = "name";
                    j += 1;
                }
                Tok::Lpar=>{
                    prev_type = "lpar";
                    j += 1;
                }
                _ => {
                    return j;
                }
            }
        }
        else if prev_type == "minus" {
            match token {
                Tok::Float { value: _ } => {
                    prev_type = "float";
                    j += 1;
                }
                Tok::Name { name: _ } => {
                    prev_type = "name";
                    j += 1;
                }
                Tok::Lpar => {
                    prev_type = "lpar";
                    j += 1;
                }
                _ => {
                    return j;
                }
            }
        }
        else if prev_type == "name" {
            match token {
                Tok::Lpar => {
                    prev_type = "lpar";
                    j += 1;
                }
                Tok::Dot => {
                    prev_type = "dot";
                    j += 1;
                }
                Tok::Equal => {
                    prev_type = "equal";
                    j += 1;
                }
                Tok::PlusEqual => {
                    prev_type = "plus_equal";
                    j += 1;
                }
                Tok::Less => {
                    prev_type = "less";
                    j += 1;
                }
                Tok::PlusPlus => {
                    prev_type = "plus_plus";
                    j += 1;
                }
                Tok::Plus => {
                    prev_type = "plus";
                    j += 1;
                }
                _ => {
                    return j;
                }
            }
        }
        else if prev_type == "dot" {
            match token {
                Tok::Name { name: _ } => {
                    prev_type = "name";
                    j += 1;
                }
                _ => {
                    return j;
                }
            }
        }
        else if prev_type == "lpar" {
            let k = find_matching_paren(j - 1, tokens);
            j = k + 1;
            prev_type = "rpar";
        }
        else if prev_type == "rpar" {
            match token {
                Tok::Lpar => {
                    let k = find_matching_paren(j - 1, tokens);
                    j = k + 1;
                    prev_type = "rpar";
                }
                Tok::EndOfLine => {
                    return j - 1;
                }
                Tok::Dot => {
                    prev_type = "dot";
                    j += 1;
                }
                Tok::Semi => {
                    return j;
                }
                Tok::Plus => {
                    prev_type = "plus";
                    j += 1;
                }
                Tok::Minus => {
                    prev_type = "minus";
                    j += 1;
                }
                Tok::Bslash => {
                    prev_type = "bslash";
                    j += 1;
                }
                _ => {
                    return j;
                }
            }
        }
        else if prev_type == "string" {
            match token {
                Tok::Semi | Tok::EndOfLine => {
                    return j;
                }
                _ => {
                    j += 1;
                }
            }
        }
        else {
            j += 1;
        }
    }
    if j == tokens.len() {
        return j - 1;
    }
    return j;
}