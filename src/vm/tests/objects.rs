use std::fs;
use crate::lexer::lexer::Lexer;
use crate::parser::parser::Parser;
use crate::compiler::compiler::Compiler;
use crate::vm::vm::Vm;
use crate::parser::symbols::JSItem;

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

    let mut vm = Vm::new();
    let out = vm.run(com.bc_ins);

    assert_eq!(out, JSItem::Undefined);

    assert_eq!(vm.captured_output, vec![
        vec![JSItem::Object { mutable: true, properties: hashmap!{
            "a".to_string() =>  JSItem::ObjectReference { path: vec![String::from("0"), String::from("a"), String::from("a")] },
            "b".to_string() => JSItem::ObjectReference { path: vec![String::from("0"), String::from("a"), String::from("b")] },
            "d".to_string() => JSItem::ObjectReference { path: vec![String::from("0"), String::from("a"), String::from("d")] }
        } }]
    ]);
}