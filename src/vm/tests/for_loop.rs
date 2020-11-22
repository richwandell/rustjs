use std::fs;

use crate::compiler::compiler::Compiler;
use crate::lexer::lexer::Lexer;
use crate::parser::parser::Parser;
use crate::parser::symbols::{JSItem};
use crate::vm::vm::Vm;

#[test]
fn test_simple_for() {
    let file = fs::read_to_string("js/if_while_for/for1.js");

    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(file.unwrap());
    let mut js_items = parser.parse(tokens);

    let mut compiler = Compiler::new();
    compiler.compile(js_items.pop().unwrap());

    let mut vm = Vm::new();
    let out = vm.run(compiler.bc_ins);

    assert_eq!(out, JSItem::Undefined);
    let captured = vm.captured_output;
    assert_eq!(captured.len(), 10);
    assert!(captured.eq(&vec![
        vec![JSItem::Number {value: 0.}],
        vec![JSItem::Number {value: 1.}],
        vec![JSItem::Number {value: 2.}],
        vec![JSItem::Number {value: 3.}],
        vec![JSItem::Number {value: 4.}],
        vec![JSItem::Number {value: 5.}],
        vec![JSItem::Number {value: 6.}],
        vec![JSItem::Number {value: 7.}],
        vec![JSItem::Number {value: 8.}],
        vec![JSItem::Number {value: 9.}],
    ]))
}

#[test]
fn test_nested_for() {
    let file = fs::read_to_string("js/if_while_for/nested_for.js");

    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(file.unwrap());
    let mut js_items = parser.parse(tokens);

    let mut compiler = Compiler::new();
    compiler.compile(js_items.pop().unwrap());

    let mut vm = Vm::new();
    let out = vm.run(compiler.bc_ins);

    assert_eq!(out, JSItem::Undefined);
    let captured = vm.captured_output;
    assert_eq!(captured.len(), 25);
    assert!(captured.eq(&vec![
        vec![JSItem::Number {value: 0.}, JSItem::Number {value: 0.}],
        vec![JSItem::Number {value: 0.}, JSItem::Number {value: 1.}],
        vec![JSItem::Number {value: 0.}, JSItem::Number {value: 2.}],
        vec![JSItem::Number {value: 0.}, JSItem::Number {value: 3.}],
        vec![JSItem::Number {value: 0.}, JSItem::Number {value: 4.}],
        vec![JSItem::Number {value: 1.}, JSItem::Number {value: 0.}],
        vec![JSItem::Number {value: 1.}, JSItem::Number {value: 1.}],
        vec![JSItem::Number {value: 1.}, JSItem::Number {value: 2.}],
        vec![JSItem::Number {value: 1.}, JSItem::Number {value: 3.}],
        vec![JSItem::Number {value: 1.}, JSItem::Number {value: 4.}],
        vec![JSItem::Number {value: 2.}, JSItem::Number {value: 0.}],
        vec![JSItem::Number {value: 2.}, JSItem::Number {value: 1.}],
        vec![JSItem::Number {value: 2.}, JSItem::Number {value: 2.}],
        vec![JSItem::Number {value: 2.}, JSItem::Number {value: 3.}],
        vec![JSItem::Number {value: 2.}, JSItem::Number {value: 4.}],
        vec![JSItem::Number {value: 3.}, JSItem::Number {value: 0.}],
        vec![JSItem::Number {value: 3.}, JSItem::Number {value: 1.}],
        vec![JSItem::Number {value: 3.}, JSItem::Number {value: 2.}],
        vec![JSItem::Number {value: 3.}, JSItem::Number {value: 3.}],
        vec![JSItem::Number {value: 3.}, JSItem::Number {value: 4.}],
        vec![JSItem::Number {value: 4.}, JSItem::Number {value: 0.}],
        vec![JSItem::Number {value: 4.}, JSItem::Number {value: 1.}],
        vec![JSItem::Number {value: 4.}, JSItem::Number {value: 2.}],
        vec![JSItem::Number {value: 4.}, JSItem::Number {value: 3.}],
        vec![JSItem::Number {value: 4.}, JSItem::Number {value: 4.}],
    ]))
}

#[test]
fn test_function_nested_for() {
    let file = fs::read_to_string("js/functions/function_with_loop.js");

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
    let captured = vm.captured_output;
    assert_eq!(captured.len(), 25);
    assert!(captured.eq(&vec![
        vec![JSItem::Number {value: 0.}, JSItem::Number {value: 0.}],
        vec![JSItem::Number {value: 0.}, JSItem::Number {value: 1.}],
        vec![JSItem::Number {value: 0.}, JSItem::Number {value: 2.}],
        vec![JSItem::Number {value: 0.}, JSItem::Number {value: 3.}],
        vec![JSItem::Number {value: 0.}, JSItem::Number {value: 4.}],
        vec![JSItem::Number {value: 1.}, JSItem::Number {value: 0.}],
        vec![JSItem::Number {value: 1.}, JSItem::Number {value: 1.}],
        vec![JSItem::Number {value: 1.}, JSItem::Number {value: 2.}],
        vec![JSItem::Number {value: 1.}, JSItem::Number {value: 3.}],
        vec![JSItem::Number {value: 1.}, JSItem::Number {value: 4.}],
        vec![JSItem::Number {value: 2.}, JSItem::Number {value: 0.}],
        vec![JSItem::Number {value: 2.}, JSItem::Number {value: 1.}],
        vec![JSItem::Number {value: 2.}, JSItem::Number {value: 2.}],
        vec![JSItem::Number {value: 2.}, JSItem::Number {value: 3.}],
        vec![JSItem::Number {value: 2.}, JSItem::Number {value: 4.}],
        vec![JSItem::Number {value: 3.}, JSItem::Number {value: 0.}],
        vec![JSItem::Number {value: 3.}, JSItem::Number {value: 1.}],
        vec![JSItem::Number {value: 3.}, JSItem::Number {value: 2.}],
        vec![JSItem::Number {value: 3.}, JSItem::Number {value: 3.}],
        vec![JSItem::Number {value: 3.}, JSItem::Number {value: 4.}],
        vec![JSItem::Number {value: 4.}, JSItem::Number {value: 0.}],
        vec![JSItem::Number {value: 4.}, JSItem::Number {value: 1.}],
        vec![JSItem::Number {value: 4.}, JSItem::Number {value: 2.}],
        vec![JSItem::Number {value: 4.}, JSItem::Number {value: 3.}],
        vec![JSItem::Number {value: 4.}, JSItem::Number {value: 4.}],
    ]))
}
//
// #[test]
// fn test_function_for_scope() {
//     let file = fs::read_to_string("js/functions/function_for_scope.js");
//
//     let mut lex = Lexer::new();
//     let mut parser = Parser::new();
//     let tokens = lex.lex(file.unwrap());
//     let mut js_items = parser.parse(tokens);
//
//     let mut int = Interpreter::new();
//     let mut out = JSItem::Undefined;
//     for item in js_items {
//         out = int.interpret(item);
//     }
//
//     assert_eq!(out, JSItem::Undefined);
//     let captured = int.captured_output;
//     assert_eq!(captured.len(), 25);
//     assert!(captured.eq(&vec![
//         vec![JSItem::Number {value: 5.}, JSItem::Number {value: 5.}],
//         vec![JSItem::Number {value: 5.}, JSItem::Number {value: 6.}],
//         vec![JSItem::Number {value: 5.}, JSItem::Number {value: 7.}],
//         vec![JSItem::Number {value: 5.}, JSItem::Number {value: 8.}],
//         vec![JSItem::Number {value: 5.}, JSItem::Number {value: 9.}],
//         vec![JSItem::Number {value: 6.}, JSItem::Number {value: 5.}],
//         vec![JSItem::Number {value: 6.}, JSItem::Number {value: 6.}],
//         vec![JSItem::Number {value: 6.}, JSItem::Number {value: 7.}],
//         vec![JSItem::Number {value: 6.}, JSItem::Number {value: 8.}],
//         vec![JSItem::Number {value: 6.}, JSItem::Number {value: 9.}],
//         vec![JSItem::Number {value: 7.}, JSItem::Number {value: 5.}],
//         vec![JSItem::Number {value: 7.}, JSItem::Number {value: 6.}],
//         vec![JSItem::Number {value: 7.}, JSItem::Number {value: 7.}],
//         vec![JSItem::Number {value: 7.}, JSItem::Number {value: 8.}],
//         vec![JSItem::Number {value: 7.}, JSItem::Number {value: 9.}],
//         vec![JSItem::Number {value: 8.}, JSItem::Number {value: 5.}],
//         vec![JSItem::Number {value: 8.}, JSItem::Number {value: 6.}],
//         vec![JSItem::Number {value: 8.}, JSItem::Number {value: 7.}],
//         vec![JSItem::Number {value: 8.}, JSItem::Number {value: 8.}],
//         vec![JSItem::Number {value: 8.}, JSItem::Number {value: 9.}],
//         vec![JSItem::Number {value: 9.}, JSItem::Number {value: 5.}],
//         vec![JSItem::Number {value: 9.}, JSItem::Number {value: 6.}],
//         vec![JSItem::Number {value: 9.}, JSItem::Number {value: 7.}],
//         vec![JSItem::Number {value: 9.}, JSItem::Number {value: 8.}],
//         vec![JSItem::Number {value: 9.}, JSItem::Number {value: 9.}],
//     ]))
// }