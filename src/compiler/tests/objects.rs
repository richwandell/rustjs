use std::fs;
use crate::lexer::lexer::Lexer;
use crate::parser::parser::Parser;
use crate::compiler::compiler::Compiler;
use crate::compiler::op_codes::Op;

#[test]
fn test_object_new_property() {
    let file = fs::read_to_string("js/objects/object_new_property.js");

    let mut lex = Lexer::new();
    let tokens = lex.lex(file.unwrap());
    let mut parser = Parser::new();
    let mut js_items = parser.parse(tokens);

    let mut com = Compiler::new();
    for item in js_items {
        com.compile(item);
    }

    assert!(com.bc_ins == vec![
        Op::CreateObj,
        Op::LoadNumConst {value: 1.},
        Op::StoreProp { name: "a".to_string()},
        Op::LoadNumConst {value: 2.},
        Op::StoreProp {name: "b".to_string() },
        Op::Store {name: "a".to_string()},
        Op::LoadStrConst {value: "hello world".to_string()},
        Op::Load {name: "a".to_string()},
        Op::StoreProp {name: "d".to_string() },
        Op::Load {name: "console".to_string()},
        Op::LoadProp {name: "log".to_string()},
        Op::Load {name: "a".to_string()},
        Op::Call {args: 1}
    ] || com.bc_ins == vec![
        Op::CreateObj,
        Op::LoadNumConst {value: 2.},
        Op::StoreProp {name: "b".to_string() },
        Op::LoadNumConst {value: 1.},
        Op::StoreProp { name: "a".to_string() },
        Op::Store {name: "a".to_string()},
        Op::LoadStrConst {value: "hello world".to_string()},
        Op::Load {name: "a".to_string()},
        Op::StoreProp {name: "d".to_string() },
        Op::Load {name: "console".to_string()},
        Op::LoadProp {name: "log".to_string()},
        Op::Load {name: "a".to_string()},
        Op::Call {args: 1}
    ]);
}