use std::fs;

use crate::compiler::compiler::Compiler;
use crate::compiler::op_codes::Op;
use crate::lexer::lexer::Lexer;
use crate::parser::parser::Parser;

#[test]
fn test_let_variable_declaration() {
    let file = fs::read_to_string("js/variables/let_a.js");

    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(file.unwrap());
    let mut js_items = parser.parse(tokens);

    let mut com = Compiler::new();
    let item = js_items.pop().unwrap();
    com.compile(item);

    assert_eq!(com.bc_ins, vec![
        Op::LoadStrConst {
            value: "hi".to_string()
        },
        Op::Store {
            name: "a".to_string()
        }
    ]);
}

#[test]
fn test_let_variable_declaration_number() {
    let file = fs::read_to_string("js/variables/let_a_number.js");

    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(file.unwrap());
    let mut js_items = parser.parse(tokens);

    let mut com = Compiler::new();
    let item = js_items.pop().unwrap();
    com.compile(item);

    assert_eq!(com.bc_ins, vec![
        Op::LoadNumConst {
            value: 123.
        },
        Op::Store {
            name: "a".to_string()
        }
    ]);
}

#[test]
fn test_let_math_declaration() {
    let file = fs::read_to_string("js/variables/let_a_math.js");

    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(file.unwrap());
    let mut js_items = parser.parse(tokens);

    let mut com = Compiler::new();
    let item = js_items.pop().unwrap();
    com.compile(item);

    assert_eq!(com.bc_ins, vec![
        Op::LoadNumConst {
            value: 1.
        },
        Op::LoadNumConst {
            value: 2.
        },
        Op::Add,
        Op::Store {
            name: "a".to_string()
        }
    ]);
}

#[test]
fn test_let_math_declaration1() {
    let file = fs::read_to_string("js/variables/let_a_math1.js");

    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(file.unwrap());
    let mut js_items = parser.parse(tokens);

    let mut com = Compiler::new();
    let item = js_items.pop().unwrap();
    com.compile(item);

    assert_eq!(com.bc_ins, vec![
        Op::LoadNumConst {
            value: 1.
        },
        Op::LoadNumConst {
            value: 2.
        },
        Op::LoadNumConst {
            value: 3.
        },
        Op::LoadNumConst {
            value: 2.
        },
        Op::Add,
        Op::Mul,
        Op::Add,
        Op::Store {
            name: "a".to_string()
        }
    ]);
}

#[test]
fn test_let_var_plus_var() {
    let file = fs::read_to_string("js/variables/let_var_plus_var.js");

    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(file.unwrap());
    let mut js_items = parser.parse(tokens);

    let mut com = Compiler::new();
    let item = js_items.pop().unwrap();
    com.compile(item);

    assert_eq!(com.bc_ins, vec![
        Op::Load {
            name: "a".to_string()
        },
        Op::Load {
            name: "b".to_string()
        },
        Op::Add,
        Op::Store {name: "c".to_string()}
    ]);
}