mod ast;
mod lexer;
mod tests;

use std::{env, fs};
use std::str::Chars;
use crate::lexer::find_token::find_token;
use crate::lexer::lexer::Lexer;


fn main() {
    let args: Vec<String> = env::args().collect();

    let f = "1.".parse::<f64>();
    println!("{:?}", f);

    if args[1] == "-f" {
        let file = fs::read_to_string(&args[2]);

        match file {
            Ok(file) => {
                let mut lex = Lexer::new();
                lex.lex(file);
                lex.test();
            }
            Err(e) => println!("{:?}", e)
        }
    }
}
