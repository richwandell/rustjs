use crate::js_token::JsToken;

pub struct Scope {
    tokens: Vec<Box<dyn JsToken>>
}

impl Scope {
    pub fn new() -> Scope {
        Scope {
            tokens: Vec::new()
        }
    }

    pub(crate) fn add_token(&mut self, token: Box<dyn JsToken>) {
        self.tokens.push(token);
    }

    pub fn test(&self) {
        for token in self.tokens.iter() {
            token.print_name();
        }
    }
}