use std::str::Chars;
use crate::js_token::{Tok};
use std::error::Error;
use crate::{constants};
use std::convert::TryFrom;
use crate::constants::ParseError;

fn find_float(it: &mut Chars, ch: char) -> Result<Vec<Tok>, ParseError> {
    let mut word = String::from("");
    word.push(ch);

    let end_chars = vec![' ', ';'];

    loop {
        let cho = it.next();
        if cho != None {
            let ch = cho.unwrap();


            if word.len() > 0 && end_chars.contains(&ch) {
                break
            }

            word.push(ch);
        }
    }
    let f = word.parse::<f64>();
    if f.is_err() {
        return Err(ParseError::Error{text: String::from("Invalid Float Value")})
    }
    return Ok(vec![Tok::Float {value: f.unwrap()}])
}

fn find_string_double_quote(it: &mut Chars) -> Result<Vec<Tok>, ParseError> {
    let mut word = String::from("");
    loop {
        let cho = it.next();
        if cho != None {
            let ch = cho.unwrap();

            if word.len() > 0 && ch == '"' {
                break
            }

            word.push(ch);
        }
    }
    return Ok(vec![Tok::String {value: word}])
}

fn find_equal(it: &mut Chars) -> Result<Vec<Tok>, ParseError> {
    let mut word = String::from("=");
    loop {
        let cho = it.next();
        if cho != None {
            let ch = cho.unwrap();
            if ch != ' ' {
                word.push(ch);
            }

            if word.len() > 0 && ch == ' ' {
                break
            }
        }
    }

    word = word.trim().parse().unwrap();
    if word == "=" {
        return Ok(vec![Tok::Equal]);
    }
    if word == "==" {
        return Ok(vec![Tok::EqEqual]);
    }
    if word == "===" {
        return Ok(vec![Tok::EqEqEual]);
    }
    if word == "=>" {
        return Ok(vec![Tok::RdoubleArrow])
    }
    return Err(ParseError::Error{text: String::from("Equals Error")})
}

fn find_let(it: &mut Chars) -> Result<Vec<Tok>, ParseError> {
    let mut word = String::from("");
    loop {
        let cho = it.next();
        if cho != None {
            let ch = cho.unwrap();
            if ch != ' ' {
                word.push(ch);
            }

            if word.len() > 0 && ch == ' ' {
                break
            }
        }
    }
    word = word.trim().parse().unwrap();
    return Ok(vec![Tok::Let, Tok::Name {name: word}])
}

pub fn find_token(it: &mut Chars) -> Result<Vec<Tok>, ParseError> {

    let mut word = String::from("");

    loop {
        let cho = it.next();
        if cho != None {
            let ch = cho.unwrap();

            if ch == '.' && word.len() > 0 {
                return Ok(vec![Tok::Name {name: word}, Tok::Dot]);
            }

            if ch == ' ' && word.len() > 0 {
                if word == "let" {
                    return find_let(it);
                }
                return Ok(vec![Tok::Name {name: word}])
            }

            if ch == '(' {
                if word.len() > 0 {
                    return Ok(vec![Tok::Name {name: word}, Tok::Lpar])
                }
                return Ok(vec![Tok::Lpar]);
            }

            if ch != ' ' && ch != '\n' {
                word.push(ch);
            }

            if ch == '=' {
                return find_equal(it);
            }

            if ch == '"' {
                return find_string_double_quote(it);
            }

            if ch == ';' {
                return Ok(vec![Tok::Semi]);
            }

            if ch == ')' {
                return Ok(vec![Tok::Rpar]);
            }

            if ch == '{' {
                return Ok(vec![Tok::Lbrace]);
            }

            if ch == '}' {
                return Ok(vec![Tok::Rbrace]);
            }

            if ch.is_numeric() {
                return find_float(it, ch);
            }

        } else {
            return Err(ParseError::Error{text: String::from("Unknown Parse Error")});
        }
    }
}