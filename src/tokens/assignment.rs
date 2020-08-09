use std::str::Chars;
use std::error::Error;
use crate::js_token::JsToken;
use crate::constants;

pub struct Assignment {
    pub(crate) name: String
}

impl JsToken for Assignment {

    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn print_name(&self) {
        println!("{}", self.name)
    }

    fn get_type(&self) -> String {
        return "assign".parse().unwrap();
    }
}

pub fn fill_assignment_token(it: &mut Chars, token: &Assignment) -> Result<Box<bool>, Box<dyn Error>> {
    let cho = it.next();

    if cho != None {
        let ch = cho.unwrap();
        if ch == ' ' {
            return Ok(Box::from(true));
        } else {
            return Err(Box::from(format!("unexpected {}, expected space.", ch)));
        }
    } else {
        return Err(Box::from("unknown error"));
    }
}