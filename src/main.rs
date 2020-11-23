#![allow(unused_mut)]

mod lexer;
mod parser;
mod compiler;
mod ast_interpreter;
mod vm;

use std::{fs};
use crate::lexer::lexer::Lexer;
use clap::{App, Arg};
use crate::parser::parser::Parser;
use crate::ast_interpreter::interpreter::Interpreter;
use crate::parser::symbols::JSItem;
use crate::compiler::compiler::Compiler;
use crate::compiler::to_bytes::to_bytes;
use std::fs::File;
use std::io::{Write, Read};
use crate::vm::vm::Vm;

extern crate clap;
#[macro_use] extern crate maplit;

fn get_js_items(file_name: &str) -> Vec<JSItem> {
    match  fs::read_to_string(file_name) {
        Ok(code) => {
            let mut lex = Lexer::new();
            let mut parser = Parser::new();
            let tokens = lex.lex(code);
            let mut js_items = parser.parse(tokens);
            return js_items;
        }
        Err(e) => {
            println!("{:?}", e);
            panic!()
        }
    }
}

fn compile(file_name: &str, output_file: &str) {
    let js_items = get_js_items(file_name);
    let mut com = Compiler::new();
    for item in js_items {
        com.compile(item);
    }
    let bytes = to_bytes(com.bc_ins);
    let mut file = File::create(output_file).unwrap();
    file.write_all(&bytes);
}

fn run_bytes(file_name: &str) {
    let mut file = File::open(file_name).unwrap();
    let mut buffer = Vec::<u8>::new();
    file.read_to_end(&mut buffer);
    println!("{:?}", buffer);
}

fn run(file_name: &str) {
    let js_items = get_js_items(file_name);
    let mut compiler = Compiler::new();
    for item in js_items {
        compiler.compile(item);
    }
    let mut vm = Vm::new();
    let out = vm.run(compiler.bc_ins);
    match out {
        JSItem::Null | JSItem::Undefined => {}
        _ => println!("{}", out)
    }
}

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
        .arg(Arg::with_name("compile")
            .help("Compile to bytecode")
            .long("compile")
            .short("c")
            .requires_all(&["outputfile"])
            .required(false))
        .arg(Arg::with_name("outputfile")
            .help("Output file name")
            .long("outputfile")
            .short("o")
            .requires_all(&["compile"])
            .takes_value(true))
        .arg(Arg::with_name("bytes")
            .help("Run compiled byte file")
            .long("bytes")
            .short("b")
            .requires_all(&["file"])
            .takes_value(false))
        .get_matches();

    let file_name = matches.value_of("file").unwrap();

    if matches.is_present("compile") {
        compile(file_name, matches.value_of("outputfile").unwrap());
    } else if matches.is_present("bytes") {
        run_bytes(file_name);
    } else {
        run(file_name);
    }
}
