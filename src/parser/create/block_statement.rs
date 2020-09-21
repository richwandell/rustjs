use crate::parser::symbols::JSItem;
use crate::lexer::js_token::Tok;
use crate::parser::create::comma_separate_tokens;
use crate::parser::parser::{Parser, SyntaxError};
use std::collections::HashMap;
use crate::parser::parser::SyntaxError::UnexpectedToken;

pub(crate) fn create_object_expression(mut tokens: Vec<Tok>) -> Result<JSItem, SyntaxError> {
    //get rid of braces
    tokens.remove(0);
    tokens.pop();
    tokens.reverse();

    let mut items = comma_separate_tokens(tokens);

    let mut object = HashMap::new();

    for mut item in items {

        if let Tok::EndOfLine = item.get(0).unwrap() {
            item.remove(0);
        }

        let mut key ;
        let tok = item.remove(0);
        match tok {
            Tok::Name {name} => {
                key = name;
            }
            Tok::String {value} => {
                key = value;
            }
            _ => {
                return Err(SyntaxError::UnexpectedToken {tok})
            }
        }

        //get rid of colon
        let colon = item.remove(0);
        match colon {
            Tok::Colon => {},
            _ => {
                return Err(UnexpectedToken {tok: colon})
            }
        }

        let mut p = Parser::new();
        let mut value = p.parse(item);
        object.insert(key, value.pop().unwrap());
    }

    Ok(JSItem::Object {
        mutable: true,
        properties: object
    })
}