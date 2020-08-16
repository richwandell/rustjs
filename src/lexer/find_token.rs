use std::str::Chars;
use std::error::Error;
use std::convert::TryFrom;
use crate::lexer::lexer::LexError;
use crate::lexer::string_iterator::StringIterator;
use crate::lexer::js_token::Tok;

fn find_float(it: &mut StringIterator, ch: char) -> Result<Vec<Tok>, LexError> {
    let mut word = String::from("");
    word.push(ch);

    loop {
        let ch = it.next();
        match ch {
            Ok(ch) => {
                if !ch.is_numeric() && ch != '.' {
                    it.prev();
                    break;
                }

                word.push(ch);
            }
            Err(e) => {
                break
            }
        }
    }
    let f = word.parse::<f64>();
    if f.is_err() {
        return Err(LexError::Error { text: String::from("Invalid Float Value") });
    }
    return Ok(vec![Tok::Float { value: f.unwrap() }]);
}

fn find_string_double_quote(it: &mut StringIterator) -> Result<Vec<Tok>, LexError> {
    let mut word = String::from("");
    loop {
        let ch = it.next();
        match ch {
            Ok(ch) => {
                if word.len() > 0 && ch == '"' {
                    break;
                }

                word.push(ch);
            }
            Err(e) => {
                break
            }
        }
    }
    return Ok(vec![Tok::String { value: word }]);
}

fn find_equal(it: &mut StringIterator) -> Result<Vec<Tok>, LexError> {
    let mut word = String::from("=");

    loop {
        let ch = it.next();
        match ch {
            Ok(ch) => {
                if ch != '=' && ch != '>' {
                    it.prev();
                    break
                }

                word.push(ch);
            }
            Err(e) => {
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
        return Ok(vec![Tok::RdoubleArrow]);
    }
    return Err(LexError::Error { text: String::from("Equals Error") });
}

fn find_let(it: &mut StringIterator) -> Result<Vec<Tok>, LexError> {
    let mut word = String::from("");
    loop {
        let ch = it.next();
        match ch {
            Ok(ch) => {
                if ch != ' ' {
                    word.push(ch);
                }

                if word.len() > 0 && ch == ' ' {
                    break;
                }
            }
            Err(e) => {
                break
            }
        }
    }
    word = word.trim().parse().unwrap();
    return Ok(vec![Tok::Let, Tok::Name { name: word }]);
}

fn find_const(it: &mut StringIterator) -> Result<Vec<Tok>, LexError> {
    let mut word = String::from("");
    loop {
        let ch = it.next();
        match ch {
            Ok(ch) => {
                if ch != ' ' {
                    word.push(ch);
                }

                if word.len() > 0 && ch == ' ' {
                    break;
                }
            }
            Err(e) => {
                break
            }
        }
    }
    word = word.trim().parse().unwrap();
    return Ok(vec![Tok::Const, Tok::Name { name: word }]);
}

fn find_bitwise(it: &mut StringIterator, ch: char) -> Result<Vec<Tok>, LexError> {
    let mut word = String::from("");
    word.push(ch);

    loop {
        let ch = it.next();
        match ch {
            Ok(ch) => {
                if ch != '<' && ch != '>' && ch != '=' {
                    it.prev();
                    break
                }
                word.push(ch);
            }
            Err(e) => {
                break
            }
        }
    }
    word = word.trim().parse().unwrap();
    if word == "<<" {
        return Ok(vec![Tok::LeftShift]);
    }
    if word == ">>" {
        return Ok(vec![Tok::RightShift]);
    }
    if word == "<<=" {
        return Ok(vec![Tok::LeftShiftEqual]);
    }
    if word == ">>=" {
        return Ok(vec![Tok::RightShiftEqual]);
    }
    if word == ">>>" {
        return Ok(vec![Tok::RightShiftUnsigned]);
    }
    if word == ">>>=" {
        return Ok(vec![Tok::RightShiftUnsignedEqual]);
    }
    return Err(LexError::Error { text: String::from("Bitwise error") });
}

pub fn find_token(it: &mut StringIterator) -> Result<Vec<Tok>, LexError> {
    let mut word = String::from("");

    loop {
        let ch = it.next();
        match ch {
            Ok(ch) => {
                if ch == '.' && word.len() > 0 {
                    return Ok(vec![Tok::Name { name: word }, Tok::Dot]);
                }

                if ch == '(' {
                    if word.len() > 0 {
                        if word == "function" {
                            return Ok(vec![Tok::Function, Tok::Lpar]);
                        }
                        return Ok(vec![Tok::Name { name: word }, Tok::Lpar]);
                    }
                    return Ok(vec![Tok::Lpar]);
                }

                if ch == ' ' && word.len() > 0 {
                    if word == "let" {
                        return find_let(it);
                    }
                    if word == "const" {
                        return find_const(it);
                    }
                    if word == "return" {
                        return Ok(vec![Tok::Return]);
                    }
                    if word == "function" {
                        return Ok(vec![Tok::Function]);
                    }
                    return Ok(vec![Tok::Name { name: word }]);
                }

                if ch == '=' {
                    return find_equal(it);
                }

                if ch == '"' {
                    return find_string_double_quote(it);
                }

                if ch.is_numeric() {
                    return find_float(it, ch);
                }

                if ch == '>' || ch == '<' {
                    let result = find_bitwise(it, ch);
                    match result {
                        Ok(mut tokens) => {
                            if word.len() > 0 {
                                tokens.insert(0, Tok::Name {name: String::from(&word)});
                            }
                            return Ok(tokens)
                        }
                        Err(e) => {
                            return Err(e)
                        }
                    }
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

                if ch == '+' {
                    return Ok(vec![Tok::Plus]);
                }

                if ch == '-' {
                    return Ok(vec![Tok::Minus]);
                }

                if ch == '*' {
                    return Ok(vec![Tok::Star]);
                }

                if ch != ' ' && ch != '\n' && ch != '\r' {
                    word.push(ch);
                }
            }
            Err(e) => {
                return Err(LexError::End);
            }
        }
    }
}