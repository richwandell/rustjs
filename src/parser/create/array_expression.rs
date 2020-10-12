use crate::lexer::js_token::Tok;
use crate::parser::symbols::{JSItem, Expression};
use crate::parser::parser::{SyntaxError, Parser};
use crate::parser::create::comma_separate_tokens;

pub(crate) fn create_array_expression(mut tokens: Vec<Tok>) -> Result<JSItem, SyntaxError> {
    //get rid of lsqb
    tokens.remove(0);
    //get rid of eol and semi if they are on the end
    loop {
        //get rid of EOL if it exists, we don't need it at this point.
        if let Tok::EndOfLine = tokens.get(tokens.len() - 1).unwrap() {
            tokens.pop();
        } else if let Tok::Semi = tokens.get(tokens.len() - 1).unwrap() {
            tokens.pop();
        } else {
            break;
        }
    }
    //get rid of rsqb
    tokens.pop();
    tokens.reverse();

    let mut items = comma_separate_tokens(tokens);
    let mut array = vec![];

    for mut item in items {
        loop {
            //get rid of EOL if it exists, we don't need it at this point.
            if let Tok::EndOfLine = item.get(item.len() - 1).unwrap() {
                item.pop();
            } else if let Tok::Semi = item.get(item.len() - 1).unwrap() {
                item.pop();
            } else {
                break;
            }
        }

        loop {
            //get rid of EOL if it exists, we don't need it at this point.
            if let Tok::EndOfLine = item.get(0).unwrap() {
                item.remove(0);
            } else if let Tok::Semi = item.get(0).unwrap() {
                item.remove(0);
            } else {
                break;
            }
        }
        let mut p = Parser::new();
        let mut value = p.parse(item);
        array.push(value.pop().unwrap());
    }
    let len = JSItem::Number {value: array.len() as f64 };
    Ok(JSItem::Ex {
        expression: Box::new(Expression::ArrayExpression {
            items: array,
            properties: hashmap!{
                "length".to_string() => len
            }
        })
    })
}