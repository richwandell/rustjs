use crate::JsToken;

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
}
