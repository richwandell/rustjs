use std::fs;

use crate::compiler::compiler::Compiler;
use crate::lexer::lexer::Lexer;
use crate::parser::parser::Parser;
use crate::compiler::op_codes::Op;

#[test]
fn test_for() {
    let file = fs::read_to_string("js/if_while_for/for1.js");

    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(file.unwrap());
    let mut js_items = parser.parse(tokens);


    let mut com = Compiler::new();
    let item = js_items.pop().unwrap();
    com.compile(item);

    assert_eq!(com.bc_ins, vec![
        Op::SetupLoop,
        Op::LoadNumConst {value: 0.},
        Op::Store {name: "i".to_string()},
        Op::Load {name: "i".to_string()},
        Op::LoadNumConst {value: 10.},
        Op::Less,
        Op::PopJumpIfFalse {to: 14},
        Op::Load {name: "console".to_string()},
        Op::Load {name: "log".to_string()},
        Op::Load {name: "i".to_string()},
        Op::Call {args: 1},
        Op::Load {name: "i".to_string()},
        Op::InplaceAdd,
        Op::JumpAbsolute {to: 3},
        Op::PopBlock
    ]);
}
