use crate::lexer::lexer::LexError;
use crate::lexer::string_iterator::StringIterator;
use crate::lexer::js_token::Tok;

#[allow(unused_must_use)]
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
            Err(_) => {
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
            Err(_) => {
                break
            }
        }
    }
    return Ok(vec![Tok::String { value: word }]);
}

#[allow(unused_must_use)]
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
            Err(_) => {
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

#[allow(unused_must_use)]
fn find_plus(it: &mut StringIterator) -> Result<Vec<Tok>, LexError> {
    let mut word = String::from("+");

    loop {
        let ch = it.next();
        match ch {
            Ok(ch) => {
                if ch != '+' && ch != '=' {
                    it.prev();
                    break
                }

                word.push(ch);
            }
            Err(_) => {
                break
            }
        }
    }

    word = word.trim().parse().unwrap();
    if word == "+" {
        return Ok(vec![Tok::Plus]);
    }
    if word == "+=" {
        return Ok(vec![Tok::PlusEqual]);
    }
    if word == "++" {
        return Ok(vec![Tok::PlusPlus]);
    }

    return Err(LexError::Error { text: String::from("Plus Error") });
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
            Err(_) => {
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
            Err(_) => {
                break
            }
        }
    }
    word = word.trim().parse().unwrap();
    return Ok(vec![Tok::Const, Tok::Name { name: word }]);
}

#[allow(unused_must_use)]
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
            Err(_) => {
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

#[allow(unused_must_use)]
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
            Err(_) => {
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

#[allow(unused_must_use)]
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
            Err(_) => {
                break
            }
        }
    }
    return Ok(vec![Tok::EndOfLine])
}



pub(crate) fn find_token(it: &mut StringIterator) -> Result<Vec<Tok>, LexError> {
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
                        if word == "for" {
                            return Ok(vec![Tok::For, Tok::Lpar]);
                        }
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
                    if word == "for" {
                        return Ok(vec![Tok::For]);
                    }
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
                    if word == "null" {
                        return Ok(vec![Tok::Null]);
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
                    let result = find_end_of_line(it);
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
                    if word.len() > 0 {
                        return Ok(vec![Tok::Name {name: word}, Tok::Semi]);
                    }
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
                    let result = find_plus(it);
                    match result {
                        Ok(mut tokens) => {
                            if word.len() > 0 {
                                tokens.insert(0, Tok::Name {name: word});
                            }
                            return Ok(tokens);
                        }
                        Err(e) => {
                            return Err(e)
                        }
                    }
                }

                if ch == '-' {
                    if word.len() > 0 {
                        return Ok(vec![Tok::Name {name: word}, Tok::Minus]);
                    }
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
                    if word == "null" {
                        return Ok(vec![Tok::Null, Tok::Comma]);
                    }
                    if word.len() > 0 {
                        return Ok(vec![Tok::Name {name: word}, Tok::Comma]);
                    }
                    return Ok(vec![Tok::Comma]);
                }

                if ch != ' ' && ch != '\n' && ch != '\r' {
                    word.push(ch);
                }
            }
            Err(_) => {
                if word.len() > 0 {
                    return Ok(vec![Tok::Name {name: word}]);
                }
                return Err(LexError::End);
            }
        }
    }
}