use crate::lexer::lexer::Lexer;
use crate::parser::parser::Parser;
use crate::compiler::compiler::{Compiler, Op};

#[test]
fn test_simple_add() {
    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(String::from("1 + 2 + 3"));
    let mut js_items = parser.parse(tokens);

    let mut com = Compiler::new();
    let item = js_items.pop().unwrap();
    com.compile(item);

    assert_eq!(com.bc_ins, vec![
        Op::PushNum {
            value: 1.
        },
        Op::PushNum {
            value: 2.
        },
        Op::Add,
        Op::PushNum {
            value: 3.
        },
        Op::Add
    ]);
}

#[test]
fn test_simple_sub() {
    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(String::from("3 - 2"));
    let mut js_items = parser.parse(tokens);

    let mut com = Compiler::new();
    let item = js_items.pop().unwrap();
    com.compile(item);

    assert_eq!(com.bc_ins, vec![
        Op::PushNum {
            value: 3.
        },
        Op::PushNum {
            value: 2.
        },
        Op::Sub
    ]);
}

#[test]
fn test_simple_mul() {
    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(String::from("3 * 2"));
    let mut js_items = parser.parse(tokens);

    let mut com = Compiler::new();
    let item = js_items.pop().unwrap();
    com.compile(item);

    assert_eq!(com.bc_ins, vec![
        Op::PushNum {
            value: 3.
        },
        Op::PushNum {
            value: 2.
        },
        Op::Mul
    ]);
}

#[test]
fn test_simple_div() {
    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(String::from("3 / 2"));
    let mut js_items = parser.parse(tokens);

    let mut com = Compiler::new();
    let item = js_items.pop().unwrap();
    com.compile(item);

    assert_eq!(com.bc_ins, vec![
        Op::PushNum {
            value: 3.
        },
        Op::PushNum {
            value: 2.
        },
        Op::Div
    ]);
}

#[test]
fn test_add_sub() {
    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(String::from("3 + 2 - 1"));
    let mut js_items = parser.parse(tokens);

    let mut com = Compiler::new();
    let item = js_items.pop().unwrap();
    com.compile(item);

    assert_eq!(com.bc_ins, vec![
        Op::PushNum {
            value: 3.
        },
        Op::PushNum {
            value: 2.
        },
        Op::Add,
        Op::PushNum {
            value: 1.
        },
        Op::Sub
    ]);
}

#[test]
fn test_add_sub_expression() {
    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(String::from("(3 + 2) - 1"));
    let mut js_items = parser.parse(tokens);

    let mut com = Compiler::new();
    let item = js_items.pop().unwrap();
    com.compile(item);

    assert_eq!(com.bc_ins, vec![
        Op::PushNum {
            value: 3.
        },
        Op::PushNum {
            value: 2.
        },
        Op::Add,
        Op::PushNum {
            value: 1.
        },
        Op::Sub
    ]);
}

#[test]
fn test_sub_add_expression() {
    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(String::from("3 - (2 + 1)"));
    let mut js_items = parser.parse(tokens);

    let mut com = Compiler::new();
    let item = js_items.pop().unwrap();
    com.compile(item);

    assert_eq!(com.bc_ins, vec![
        Op::PushNum {
            value: 3.
        },
        Op::PushNum {
            value: 2.
        },
        Op::PushNum {
            value: 1.
        },
        Op::Add,
        Op::Sub
    ]);
}