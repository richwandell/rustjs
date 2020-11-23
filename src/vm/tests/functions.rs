use crate::lexer::lexer::Lexer;
use crate::parser::parser::Parser;
use crate::compiler::compiler::Compiler;
use crate::vm::vm::Vm;
use crate::parser::symbols::JSItem;
use std::fs;

#[test]
fn test_console_dot_log() {
    let mut lex = Lexer::new();
    let tokens = lex.lex(String::from("console.log(\"hi\");"));
    let mut parser = Parser::new();
    let mut js_items = parser.parse(tokens);

    let mut com = Compiler::new();
    com.compile(js_items.pop().unwrap());

    let mut vm = Vm::new();
    let out = vm.run(com.bc_ins);

    assert_eq!(out, JSItem::Undefined);

    assert_eq!(vm.captured_output, vec![
        vec![JSItem::String {value: "hi".to_string()}]
    ]);
}

#[test]
fn test_function_call() {
    let file = fs::read_to_string("js/functions/function_call.js");

    let mut lex = Lexer::new();
    let tokens = lex.lex(file.unwrap());
    let mut parser = Parser::new();
    let mut js_items = parser.parse(tokens);

    let mut com = Compiler::new();
    for item in js_items {
        com.compile(item);
    }

    let mut vm = Vm::new();
    let out = vm.run(com.bc_ins);

    assert_eq!(out, JSItem::Undefined);

    assert_eq!(vm.captured_output, vec![
        vec![JSItem::String {value: "hi".to_string()}]
    ]);
}

#[test]
fn test_function_call_with_arg() {
    let file = fs::read_to_string("js/functions/function_call_with_arg.js");

    let mut lex = Lexer::new();
    let tokens = lex.lex(file.unwrap());
    let mut parser = Parser::new();
    let mut js_items = parser.parse(tokens);

    let mut com = Compiler::new();
    for item in js_items {
        com.compile(item);
    }

    let mut vm = Vm::new();
    let out = vm.run(com.bc_ins);

    assert_eq!(out, JSItem::Undefined);

    assert_eq!(vm.captured_output, vec![
        vec![JSItem::String {value: "hi".to_string()}]
    ]);
}

#[test]
fn test_function_function_call_with_args() {
    let file = fs::read_to_string("js/functions/function_function_call_with_args.js");

    let mut lex = Lexer::new();
    let tokens = lex.lex(file.unwrap());
    let mut parser = Parser::new();
    let mut js_items = parser.parse(tokens);

    let mut com = Compiler::new();
    for item in js_items {
        com.compile(item);
    }

    let mut vm = Vm::new();
    let out = vm.run(com.bc_ins);

    assert_eq!(out, JSItem::Undefined);

    assert_eq!(vm.captured_output, vec![
        vec![JSItem::String {value: "hi".to_string()}, JSItem::String {value: "there".to_string()}]
    ]);
}

#[test]
fn test_let_function_call() {
    let file = fs::read_to_string("js/functions/let_function_call.js");

    let mut lex = Lexer::new();
    let tokens = lex.lex(file.unwrap());
    let mut parser = Parser::new();
    let mut js_items = parser.parse(tokens);

    let mut com = Compiler::new();
    for item in js_items {
        com.compile(item);
    }

    let mut vm = Vm::new();
    let out = vm.run(com.bc_ins);

    assert_eq!(out, JSItem::Undefined);

    assert_eq!(vm.captured_output, vec![
        vec![JSItem::String {value: "hi".to_string()}]
    ]);
}