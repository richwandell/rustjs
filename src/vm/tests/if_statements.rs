use std::fs;

use crate::compiler::compiler::Compiler;
use crate::lexer::lexer::Lexer;
use crate::parser::parser::Parser;
use crate::parser::symbols::JSItem;
use crate::vm::vm::Vm;

#[test]
fn test_if1() {
    let file = fs::read_to_string("js/if_while_for/if1.js");

    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(file.unwrap());
    let js_items = parser.parse(tokens);

    let mut com = Compiler::new();
    for item in js_items {
        com.compile(item);
    }

    let mut vm = Vm::new();
    let out = vm.run(com.bc_ins);

    assert_eq!(out, JSItem::Undefined);
    let captured = vm.captured_output;
    assert_eq!(captured.len(), 1);
    assert!(captured.eq(&vec![
        vec![JSItem::String {value: "hi".to_string()}]
    ]))
}

#[test]
fn test_if4() {
    let file = fs::read_to_string("js/if_while_for/if4.js");

    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(file.unwrap());
    let js_items = parser.parse(tokens);

    let mut com = Compiler::new();
    for item in js_items {
        com.compile(item);
    }

    let mut vm = Vm::new();
    let out = vm.run(com.bc_ins);

    assert_eq!(out, JSItem::Undefined);
    let captured = vm.captured_output;
    assert_eq!(captured.len(), 1);
    assert!(captured.eq(&vec![
        vec![JSItem::String {value: "gt 2".to_string()}]
    ]))
}

#[test]
fn test_if5() {
    let file = fs::read_to_string("js/if_while_for/if5.js");

    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(file.unwrap());
    let js_items = parser.parse(tokens);

    let mut com = Compiler::new();
    for item in js_items {
        com.compile(item);
    }

    let mut vm = Vm::new();
    let out = vm.run(com.bc_ins);

    assert_eq!(out, JSItem::Undefined);
    let captured = vm.captured_output;
    assert_eq!(captured.len(), 1);
    assert!(captured.eq(&vec![
        vec![JSItem::String {value: "there".to_string()}]
    ]))
}