use crate::lexer::lexer::Lexer;
use crate::parser::parser::Parser;
use crate::vm::interpreter::Interpreter;
use crate::parser::symbols::JSItem;

#[test]
fn test_simple_cdl() {
    let mut lex = Lexer::new();
    let mut parser = Parser::new();
    let tokens = lex.lex(String::from("console.log(\"hi\");"));
    let mut js_items = parser.parse(tokens);

    let mut int = Interpreter::new();
    let out = int.interpret(js_items.pop().unwrap());
    assert_eq!(out, JSItem::Null);
    let captured = int.captured_output;
    assert!(captured.eq(&vec![vec![JSItem::String {value: String::from("hi")}]]))
}