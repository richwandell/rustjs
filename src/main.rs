#![allow(unused_mut)]
mod lexer;
mod parser;
mod vm;

use std::{fs};
use crate::lexer::lexer::Lexer;
use clap::{App, Arg};
use crate::parser::parser::Parser;
use crate::vm::interpreter::Interpreter;
use crate::vm::js_output::JSOutput;

extern crate clap;

fn main() {
    let matches = App::new("Rust JS")
        .version("0.1")
        .author("Rich Wandell <richwandell@gmail.com>")
        .about("JavaScript Interpreter")
        .arg(Arg::with_name("file")
            .short("f")
            .long("file")
            .value_name("FILE")
            .help("The JS file to run")
            .required(true))
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
                    JSOutput::Null => {}
                    _ => println!("{}", out)
                }
            }
        }
        Err(e) => println!("{:?}", e)
    }
}
