use crate::js_token::JsToken;
use std::str::Chars;
use crate::find_token;
use std::any::Any;
use crate::tokens::assignment::Assignment;
use std::ops::Deref;
use crate::constants;

pub struct LetStruct {
    pub name: String
}

impl JsToken for LetStruct {
    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn print_name(&self) {
        println!("let {}", self.name);
    }

    fn get_type(&self) -> String {
        return constants::STRLET.parse().unwrap();
    }
}

pub fn fill_let_token(it: &mut Chars, let_token: &LetStruct) {
    println!("in fill let token");

    loop {
        let token = find_token(it);

        match token {
            Ok(token) => {
                token.print_name()
            }
            _ => {}
        }
        println!("{:?}", "hi");
    }
}
