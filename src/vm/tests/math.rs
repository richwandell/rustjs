use crate::lexer::lexer::Lexer;
use crate::parser::parser::Parser;
use crate::parser::symbols::JSItem;
use crate::vm::vm::Vm;
use crate::compiler::compiler::Compiler;
use std::fs;
use std::f64::{NAN, INFINITY};

#[test]
fn test_simple_add() {
    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(String::from("1 + 2 + 3"));
    let mut js_items = parser.parse(tokens);

    let mut compiler = Compiler::new();
    compiler.compile(js_items.pop().unwrap());

    let mut vm = Vm::new();
    let out = vm.run(compiler.bc_ins);
    assert_eq!(out, JSItem::Number {value: 6.});
}

#[test]
fn test_simple_sub() {
    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(String::from("3 - 2"));
    let mut js_items = parser.parse(tokens);

    let mut compiler = Compiler::new();
    compiler.compile(js_items.pop().unwrap());

    let mut vm = Vm::new();
    let out = vm.run(compiler.bc_ins);
    assert_eq!(out, JSItem::Number {value: 1.});
}

#[test]
fn test_simple_mul() {
    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(String::from("3 * 2"));
    let mut js_items = parser.parse(tokens);

    let mut compiler = Compiler::new();
    compiler.compile(js_items.pop().unwrap());

    let mut vm = Vm::new();
    let out = vm.run(compiler.bc_ins);
    assert_eq!(out, JSItem::Number {value: 6.});
}

#[test]
fn test_simple_div() {
    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(String::from("3 / 2"));
    let mut js_items = parser.parse(tokens);

    let mut compiler = Compiler::new();
    compiler.compile(js_items.pop().unwrap());

    let mut vm = Vm::new();
    let out = vm.run(compiler.bc_ins);
    assert_eq!(out, JSItem::Number {value: 1.5});
}

#[test]
fn test_add_sub() {
    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(String::from("3 + 2 - 1"));
    let mut js_items = parser.parse(tokens);

    let mut compiler = Compiler::new();
    compiler.compile(js_items.pop().unwrap());

    let mut vm = Vm::new();
    let out = vm.run(compiler.bc_ins);
    assert_eq!(out, JSItem::Number {value: 4.});
}

#[test]
fn test_add_sub_expression() {
    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(String::from("(3 + 2) - 1"));
    let mut js_items = parser.parse(tokens);

    let mut compiler = Compiler::new();
    compiler.compile(js_items.pop().unwrap());

    let mut vm = Vm::new();
    let out = vm.run(compiler.bc_ins);
    assert_eq!(out, JSItem::Number {value: 4.});
}

#[test]
fn test_sub_add_expression() {
    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(String::from("3 - (2 + 1)"));
    let mut js_items = parser.parse(tokens);

    let mut compiler = Compiler::new();
    compiler.compile(js_items.pop().unwrap());

    let mut vm = Vm::new();
    let out = vm.run(compiler.bc_ins);
    assert_eq!(out, JSItem::Number {value: 0.});
}

#[test]
fn test_add_mul() {
    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(String::from("3 + 2 * 3"));
    let mut js_items = parser.parse(tokens);

    let mut compiler = Compiler::new();
    compiler.compile(js_items.pop().unwrap());

    let mut vm = Vm::new();
    let out = vm.run(compiler.bc_ins);
    assert_eq!(out, JSItem::Number {value: 9.});
}

#[test]
fn test_mul_add() {
    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(String::from("3 * 2 + 3"));
    let mut js_items = parser.parse(tokens);

    let mut compiler = Compiler::new();
    compiler.compile(js_items.pop().unwrap());

    let mut vm = Vm::new();
    let out = vm.run(compiler.bc_ins);
    assert_eq!(out, JSItem::Number {value: 9.});
}

#[test]
fn test_mul_add_expression() {
    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(String::from("3 * (2 + 3)"));
    let mut js_items = parser.parse(tokens);

    let mut compiler = Compiler::new();
    compiler.compile(js_items.pop().unwrap());

    let mut vm = Vm::new();
    let out = vm.run(compiler.bc_ins);
    assert_eq!(out, JSItem::Number {value: 15.});
}

#[test]
fn test_mul_expression_add() {
    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(String::from("(3 * 2) + 3"));
    let mut js_items = parser.parse(tokens);

    let mut compiler = Compiler::new();
    compiler.compile(js_items.pop().unwrap());

    let mut vm = Vm::new();
    let out = vm.run(compiler.bc_ins);
    assert_eq!(out, JSItem::Number {value: 9.});
}

#[test]
fn test_and1() {
    let file = fs::read_to_string("js/math/and1.js");

    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(file.unwrap());
    let mut js_items = parser.parse(tokens);

    assert_eq!(js_items.len(), 2);

    let mut compiler = Compiler::new();
    for item in js_items {
        compiler.compile(item);
    }


    let mut vm = Vm::new();
    let out = vm.run(compiler.bc_ins);
    assert_eq!(out, JSItem::Bool {value: true});
}

#[test]
fn test_triple_equal() {
    let file = fs::read_to_string("js/math/triple_eq.js");
    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(file.unwrap());
    let mut js_items = parser.parse(tokens);

    assert_eq!(js_items.len(), 2);

    let mut compiler = Compiler::new();
    for item in js_items {
        compiler.compile(item);
    }


    let mut vm = Vm::new();
    let out = vm.run(compiler.bc_ins);
    assert_eq!(out, JSItem::Bool {value: true});
}

#[test]
fn test_div_nan1() {
    let file = fs::read_to_string("js/math/div/div_nan1.js");
    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(file.unwrap());
    let mut js_items = parser.parse(tokens);

    let mut compiler = Compiler::new();
    for item in js_items {
        compiler.compile(item);
    }

    let mut vm = Vm::new();
    let out = vm.run(compiler.bc_ins);
    assert_eq!(out, JSItem::Undefined);

    match vm.captured_output[0][0] {
        JSItem::Number {value} => {
            assert!(value.is_nan())
        }
        _ => {}
    }
    assert!(vm.captured_output[1].eq(&vec![JSItem::Number {value: 0.}]));
    assert!(vm.captured_output[2].eq(&vec![JSItem::Number {value: 0.}]));
    assert!(vm.captured_output[3].eq(&vec![JSItem::Number {value: INFINITY}]));
    assert!(vm.captured_output[4].eq(&vec![JSItem::Number {value: 1.}]));
    assert!(vm.captured_output[5].eq(&vec![JSItem::Number {value: 0.5}]));
    assert!(vm.captured_output[6].eq(&vec![JSItem::Number {value: INFINITY}]));
    assert!(vm.captured_output[7].eq(&vec![JSItem::Number {value: 2.}]));
    assert!(vm.captured_output[8].eq(&vec![JSItem::Number {value: 1.}]));
}

























