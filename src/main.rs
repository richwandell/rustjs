mod lexer;
mod parser;

use std::{env, fs};
use crate::lexer::lexer::Lexer;


fn main() {
    let args: Vec<String> = env::args().collect();

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
