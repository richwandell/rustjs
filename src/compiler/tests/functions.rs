use crate::lexer::lexer::Lexer;
use crate::parser::parser::Parser;
use crate::compiler::compiler::Compiler;
use crate::compiler::op_codes::Op;

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

