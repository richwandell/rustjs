use std::{env, fs};
use std::str::Chars;
use crate::scope::Scope;

mod js_token;
mod scope;
mod tokens;
mod constants;

use crate::tokens::find_token::find_token;
use javascript_lexer::Lexer;

fn find_name(it: &mut Chars, word: String) -> String {
    let mut name = String::from("");
    let mut cho = it.next();

    // name must be followed by a space
    if cho != None {
        let ch = cho.unwrap();
        if ch != " ".parse().unwrap() {
            let mut def = String::from(word);
            def.push(ch);
            panic!(format!("Uncaught ReferenceError: {} is not defined", def))
        }
    }

    loop {
        cho = it.next();
        if cho != None {
            let ch = cho.unwrap();
            if ch == " ".parse().unwrap() {
                if name.trim() != "" {
                    break
                }
            }
            name.push(ch);
        }
    }
    name
}

fn parse_file(file: String) {
    let mut scope = Scope::new();
    let mut it = file.chars();

    loop {
        let token = find_token(&mut it);

        match token {
            Ok(token) => {
                scope.add_token(token);
                scope.test();
            }
            Err(e) => {
                println!("{:?}", e);
                break
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);

    if args[1] == "-f" {
        let file = fs::read_to_string(&args[2]);

        match file {
            Ok(file) => {
                // parse_file(file);
                let tokens = Lexer::lex_tokens(&file);
                match tokens {
                    Ok(tokens) => {
                        for token in tokens {
                            println!("{:?}", token);
                        }
                    }
                    _ => {}
                }
            }
            Err(e) => println!("{:?}", e)
        }
    }
}
