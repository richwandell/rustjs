#![allow(unused_mut)]

mod lexer;
mod parser;
mod compiler;
mod ast_interpreter;

use std::{fs};
use crate::lexer::lexer::Lexer;
use clap::{App, Arg};
use crate::parser::parser::Parser;
use crate::ast_interpreter::interpreter::Interpreter;
use crate::parser::symbols::JSItem;

extern crate clap;
#[macro_use] extern crate maplit;

fn main() {
    let matches = App::new("Rust JS")
        .version("0.1")
        .author("Rich Wandell <richwandell@gmail.com>")
        .about("JavaScript Interpreter")
        .arg(Arg::with_name("file")
            .help("The JS file to run")
            .required(true))
        .arg(Arg::with_name("expose-gc")
            .help("Expose GP")
            .long("expose-gc")
            .required(false))
        .get_matches();

    let file_name = matches.value_of("file").unwrap();
    let js_code = fs::read_to_string(file_name);

    match js_code {
        Ok(code) => {
            let mut lex = Lexer::new();
            let mut parser = Parser::new();
            let tokens = lex.lex(code);
            let mut js_items = parser.parse(tokens);

            let mut int = Interpreter::new();
            for item in js_items {
                let out = int.interpret(item);
                match out {
                    JSItem::Null | JSItem::Undefined => {}
                    _ => println!("{}", out)
                }
            }
        }
        Err(e) => println!("{:?}", e)
    }
}
