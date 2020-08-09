mod scope;
mod keywords;
mod constants;

use keywords::{let_keyword};
use std::{env, fs};
use std::str::Chars;
use std::thread::yield_now;
use crate::keywords::let_keyword::LetStruct;
use crate::scope::Scope;

trait JsToken {
    fn set_name(&mut self, name: String);
    fn print_name(&self);
}

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

fn find_token(it: &mut Chars) -> Box<dyn JsToken> {

    let mut word = String::from("");

    loop {
        let mut cho = it.next();
        if cho != None {
            let mut ch = cho.unwrap();
            word.push(ch);

            if constants::KEYWORDS.contains(&&*word) {
                if word == constants::STRLET {
                    let name = find_name(it, word);
                    return Box::new(LetStruct{name});
                }
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
                let mut scope = Scope::new();
                let mut it = file.chars();
                let token = find_token(&mut it);

                scope.add_token(token);
                scope.test();
            }
            Err(e) => println!("{:?}", e)
        }
    }
}
