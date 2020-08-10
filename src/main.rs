use std::{env, fs};
use std::str::Chars;
use crate::lexer::{Lexer};
use crate::tokens::find_token::find_token;
use constants::ParseError;

mod js_token;
mod lexer;
mod tokens;
mod constants;


fn parse_file(file: String) {
    let mut lex = Lexer::new();
    let mut it = file.chars();

    loop {
        let token = find_token(&mut it);

        match token {
            Ok(tokens) => {
                for token in tokens {
                    lex.add_token(token);
                }
            }
            Err(e) => {
                match e {
                    ParseError::Error { text } => {
                        lex.test();
                        println!("{:?}", text);
                    }
                }
                break
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args[1] == "-f" {
        let file = fs::read_to_string(&args[2]);

        match file {
            Ok(file) => {
                parse_file(file);
            }
            Err(e) => println!("{:?}", e)
        }
    }
}
