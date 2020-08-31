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

fn find_gt_lt(it: &mut StringIterator, ch: char) -> Result<Vec<Tok>, LexError> {
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
    if word == ">" {
        return Ok(vec![Tok::Greater]);
    }
    if word == ">=" {
        return Ok(vec![Tok::GreaterEqual]);
    }
    if word == "<" {
        return Ok(vec![Tok::Less]);
    }
    if word == "<=" {
        return Ok(vec![Tok::LessEqual]);
    }
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
    return Err(LexError::Error { text: String::from("Unexpected Token") });
}

fn find_if(it: &mut StringIterator) -> Result<Vec<Tok>, LexError> {
    it.prev();
    let mut parens = vec![];

    // find the first paren
    loop {
        let ch = it.next();
        match ch {
            Ok(ch) => {
                if ch == '(' {
                    parens.push("(");
                    break;
                }
                if ch != ' ' {
                    return Err(LexError::Error { text: String::from("Syntax error") });
                }
            }
            Err(e) => {
                return Err(LexError::Error { text: String::from("Syntax error") });
            }
        }
    }

    let mut return_tokens = vec![Tok::If, Tok::Lpar];
    loop {
        let tokens = find_token(it);
        match tokens {
            Ok(tokens) => {
                for token in tokens {
                    if token == Tok::Lpar {
                        parens.push("(");
                    } else if token == Tok::Rpar {
                        parens.pop();
                    }
                    return_tokens.push(token);
                }
            }
            _ => {
                return Err(LexError::Error { text: String::from("Unexpected Token") });
            }
        }

        if parens.len() == 0 {
            break;
        }
    }
    return Ok(return_tokens);
}

fn find_end_of_line(it: &mut StringIterator) -> Result<Vec<Tok>, LexError> {
    loop {
        let ch = it.next();
        match ch {
            Ok(ch) => {
                if ch != '\r' && ch != '\n' {
                    it.prev();
                    break
                }
            }
            Err(e) => {
                break
            }
        }
    }
    return Ok(vec![Tok::EndOfLine])
}

pub fn find_token(it: &mut StringIterator) -> Result<Vec<Tok>, LexError> {
    let mut word = String::from("");

    loop {
        let ch = it.next();
        match ch {
            Ok(ch) => {
                if ch == '.' {
                    if word.len() > 0 {
                        return Ok(vec![Tok::Name { name: word }, Tok::Dot]);
                    }
                    return Ok(vec![Tok::Dot]);
                }

                if ch == '(' {
                    if word.len() > 0 {
                        if word == "function" {
                            return Ok(vec![Tok::Function, Tok::Lpar]);
                        }
                        if word == "if" {
                            return find_if(it);
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
                    if word == "if" {
                        return find_if(it);
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

                if ch == '\r' || ch == '\n' {
                    return find_end_of_line(it);
                }

                if ch == '>' || ch == '<' {
                    let result = find_gt_lt(it, ch);
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
                    if word.len() > 0 {
                        return Ok(vec![Tok::Name {name: word}, Tok::Rpar]);
                    }
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

                if ch == '/' {
                    return Ok(vec![Tok::Bslash]);
                }

                if ch == '[' {
                    return Ok(vec![Tok::Lsqb]);
                }

                if ch == ']' {
                    return Ok(vec![Tok::Rsqb]);
                }

                if ch == ',' {
                    if word.len() > 0 {
                        return Ok(vec![Tok::Name {name: word}, Tok::Comma]);
                    }
                    return Ok(vec![Tok::Comma]);
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