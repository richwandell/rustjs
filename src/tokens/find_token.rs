use std::str::Chars;
use crate::js_token::JsToken;
use std::error::Error;
use crate::{constants, find_name};
use crate::tokens::let_keyword::{LetStruct, fill_let_token};
use std::convert::TryFrom;
use crate::tokens::string_value::StringStruct;
use crate::tokens::assignment::{Assignment, fill_assignment_token};

pub fn find_token(it: &mut Chars) -> Result<Box<dyn JsToken>, Box<dyn Error>> {

    let mut word = String::from("");

    loop {
        let cho = it.next();
        if cho != None {
            let ch = cho.unwrap();
            word.push(ch);

            if ch == '=' {
                let token = Assignment{name: String::from("assign")};
                let ok = fill_assignment_token(it, &token);
                match ok {
                    Ok(ok) => return Ok(Box::new(token)),
                    Err(e) => return Err(e)
                }
            }

            if ch == '"' {
                let token = StringStruct{};
            }

            if constants::KEYWORDS.contains(&&*word) {
                if word == constants::STRLET {
                    let name = find_name(it, word);
                    let token = LetStruct{name};
                    fill_let_token(it, &token);
                    return Ok(Box::new(token));
                }
            }
        } else {
            return Err(Box::try_from("error").unwrap());
        }
    }
}