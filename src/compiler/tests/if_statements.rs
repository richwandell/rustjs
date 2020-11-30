use std::fs;
use crate::lexer::lexer::Lexer;

use crate::parser::parser::Parser;

use crate::compiler::compiler::Compiler;
use crate::compiler::op_codes::Op;

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

    assert_eq!(com.bc_ins, vec![
        Op::LoadNumConst {value: 5.},
        Op::Store {name: "x".to_string()},
        Op::Load {name: "x".to_string()},
        Op::LoadNumConst {value: 2.},
        Op::Greater,
        Op::PopJumpIfFalse {to: 11},
        Op::Load {name: "console".to_string()},
        Op::LoadProp {name: "log".to_string()},
        Op::LoadStrConst {value: "hi".to_string()},
        Op::Call {args: 1},
        Op::JumpAbsolute {to: 11}
    ]);
}

#[test]
fn test_if2() {
    let file = fs::read_to_string("js/if_while_for/if2.js");

    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(file.unwrap());
    let js_items = parser.parse(tokens);

    let mut com = Compiler::new();
    for item in js_items {
        com.compile(item);
    }

    assert_eq!(com.bc_ins, vec![
        Op::LoadNumConst {value: 5.},
        Op::Store {name: "x".to_string()},
        Op::Load {name: "x".to_string()},
        Op::LoadNumConst {value: 2.},
        Op::Greater,
        Op::PopJumpIfFalse {to: 11},
        Op::Load {name: "console".to_string()},
        Op::LoadProp {name: "log".to_string()},
        Op::LoadStrConst {value: "hi".to_string()},
        Op::Call {args: 1},
        Op::JumpAbsolute {to: 24},
        Op::Load {name: "x".to_string()},
        Op::LoadNumConst {value: 2.},
        Op::Less,
        Op::PopJumpIfFalse {to: 20},
        Op::Load {name: "console".to_string()},
        Op::LoadProp {name: "log".to_string()},
        Op::LoadStrConst {value: "gt 2".to_string()},
        Op::Call {args: 1},
        Op::JumpAbsolute {to: 24},
        Op::Load {name: "console".to_string()},
        Op::LoadProp {name: "log".to_string()},
        Op::LoadStrConst {value: "there".to_string()},
        Op::Call {args: 1},
    ]);
}

#[test]
fn test_if3() {
    let file = fs::read_to_string("js/if_while_for/if3.js");

    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(file.unwrap());
    let js_items = parser.parse(tokens);

    let mut com = Compiler::new();
    for item in js_items {
        com.compile(item);
    }

    assert_eq!(com.bc_ins, vec![
        Op::LoadNumConst {value: 5.},
        Op::Store {name: "x".to_string()},
        Op::Load {name: "x".to_string()},
        Op::LoadNumConst {value: 2.},
        Op::Greater,
        Op::PopJumpIfFalse {to: 11},
        Op::Load {name: "console".to_string()},
        Op::LoadProp {name: "log".to_string()},
        Op::LoadStrConst {value: "hi".to_string()},
        Op::Call {args: 1},
        Op::JumpAbsolute {to: 24},
        Op::Load {name: "x".to_string()},
        Op::LoadNumConst {value: 2.},
        Op::Less,
        Op::PopJumpIfFalse {to: 20},
        Op::Load {name: "console".to_string()},
        Op::LoadProp {name: "log".to_string()},
        Op::LoadStrConst {value: "gt 2".to_string()},
        Op::Call {args: 1},
        Op::JumpAbsolute {to: 24},
        Op::Load {name: "console".to_string()},
        Op::LoadProp {name: "log".to_string()},
        Op::LoadStrConst {value: "there".to_string()},
        Op::Call {args: 1},
    ]);
}