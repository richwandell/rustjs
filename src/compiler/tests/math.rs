use crate::lexer::lexer::Lexer;
use crate::parser::parser::Parser;
use crate::compiler::compiler::{Compiler};
use crate::compiler::op_codes::Op;

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
        Op::LoadNumConst {
            value: 1.
        },
        Op::LoadNumConst {
            value: 2.
        },
        Op::Add,
        Op::LoadNumConst {
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
        Op::LoadNumConst {
            value: 3.
        },
        Op::LoadNumConst {
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
        Op::LoadNumConst {
            value: 3.
        },
        Op::LoadNumConst {
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
        Op::LoadNumConst {
            value: 3.
        },
        Op::LoadNumConst {
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
        Op::LoadNumConst {
            value: 3.
        },
        Op::LoadNumConst {
            value: 2.
        },
        Op::Add,
        Op::LoadNumConst {
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
        Op::LoadNumConst {
            value: 3.
        },
        Op::LoadNumConst {
            value: 2.
        },
        Op::Add,
        Op::LoadNumConst {
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
        Op::LoadNumConst {
            value: 3.
        },
        Op::LoadNumConst {
            value: 2.
        },
        Op::LoadNumConst {
            value: 1.
        },
        Op::Add,
        Op::Sub
    ]);
}

#[test]
fn test_add_mul() {
    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(String::from("3 + 2 * 3"));
    let mut js_items = parser.parse(tokens);

    let mut com = Compiler::new();
    let item = js_items.pop().unwrap();
    com.compile(item);

    assert_eq!(com.bc_ins, vec![
        Op::LoadNumConst {
            value: 3.
        },
        Op::LoadNumConst {
            value: 2.
        },
        Op::LoadNumConst {
            value: 3.
        },
        Op::Mul,
        Op::Add
    ]);
}

#[test]
fn test_mul_add() {
    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(String::from("3 * 2 + 3"));
    let mut js_items = parser.parse(tokens);

    let mut com = Compiler::new();
    let item = js_items.pop().unwrap();
    com.compile(item);

    assert_eq!(com.bc_ins, vec![
        Op::LoadNumConst {
            value: 3.
        },
        Op::LoadNumConst {
            value: 2.
        },
        Op::Mul,
        Op::LoadNumConst {
            value: 3.
        },
        Op::Add
    ]);
}

#[test]
fn test_mul_add_expression() {
    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(String::from("3 * (2 + 3)"));
    let mut js_items = parser.parse(tokens);

    let mut com = Compiler::new();
    let item = js_items.pop().unwrap();
    com.compile(item);

    assert_eq!(com.bc_ins, vec![
        Op::LoadNumConst {
            value: 3.
        },
        Op::LoadNumConst {
            value: 2.
        },
        Op::LoadNumConst {
            value: 3.
        },
        Op::Add,
        Op::Mul
    ]);
}

#[test]
fn test_mul_expression_add() {
    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(String::from("(3 * 2) + 3"));
    let mut js_items = parser.parse(tokens);

    let mut com = Compiler::new();
    let item = js_items.pop().unwrap();
    com.compile(item);

    assert_eq!(com.bc_ins, vec![
        Op::LoadNumConst {
            value: 3.
        },
        Op::LoadNumConst {
            value: 2.
        },
        Op::Mul,
        Op::LoadNumConst {
            value: 3.
        },
        Op::Add
    ]);
}

#[test]
fn test_and1() {
    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(String::from("x == 5 && x < 10"));
    let mut js_items = parser.parse(tokens);

    assert_eq!(js_items.len(), 1);

    let mut com = Compiler::new();
    let item = js_items.pop().unwrap();
    com.compile(item);


    assert_eq!(com.bc_ins, vec![
        Op::Load { name: "x".to_string()},
        Op::LoadNumConst { value: 5. },
        Op::EqEq,
        Op::Load { name:  "x".to_string()},
        Op::LoadNumConst { value: 10. },
        Op::Less,
        Op::And
    ]);
}






















