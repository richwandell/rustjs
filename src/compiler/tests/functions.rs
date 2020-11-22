use crate::lexer::lexer::Lexer;
use crate::parser::parser::Parser;
use crate::compiler::compiler::Compiler;
use crate::compiler::op_codes::Op;
use std::fs;

#[test]
fn test_console_dot_log() {
    let mut lex = Lexer::new();
    let tokens = lex.lex(String::from("console.log(\"hi\");"));
    let mut parser = Parser::new();
    let mut js_items = parser.parse(tokens);

    let mut com = Compiler::new();
    let item = js_items.pop().unwrap();
    com.compile(item);

    assert_eq!(com.bc_ins, vec![
        Op::Load {
            name: "console".to_string()
        },
        Op::LoadProp {
            name: "log".to_string()
        },
        Op::LoadStrConst {
            value: "hi".to_string()
        },
        Op::Call {args: 1}
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

    assert_eq!(com.bc_ins, vec![
        Op::DeclareFunc {
            start: 1,
            end: 6,
            mutable: true,
            params: vec![],
            name: "f".to_string()
        },
        Op::Load {name: "console".to_string()},
        Op::LoadProp {name: "log".to_string()},
        Op::LoadStrConst {value: "hi".to_string()},
        Op::Call {args: 1},
        Op::PopBlock,
        Op::Return,
        Op::Load {name: "f".to_string()},
        Op::Call {args: 0}
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

    assert_eq!(com.bc_ins, vec![
        Op::DeclareFunc {
            start: 1,
            end: 14,
            mutable: true,
            params: vec!["a".to_string()],
            name: "f".to_string()
        },
        Op::DeclareFunc {
            start: 2,
            end: 8,
            mutable: true,
            params: vec!["a".to_string(), "b".to_string()],
            name: "f".to_string()
        },
        Op::Load {name: "console".to_string()},
        Op::LoadProp {name: "log".to_string()},
        Op::Load {name: "a".to_string()},
        Op::Load {name: "b".to_string()},
        Op::Call {args: 2},
        Op::PopBlock,
        Op::Return,
        Op::Load {name: "f".to_string()},
        Op::Load {name: "a".to_string()},
        Op::LoadStrConst {value: "there".to_string()},
        Op::Call {args: 2},
        Op::PopBlock,
        Op::Return,
        Op::Load {name: "f".to_string()},
        Op::LoadStrConst {value: "hi".to_string()},
        Op::Call {args: 1},
    ]);
}