use std::fs;
use std::any::Any;
use crate::lexer::lexer::Lexer;
use crate::lexer::js_token::Tok;

#[test]
fn test_declare_string_variable() {
    let mut lex = Lexer::new();
    let tokens = lex.lex(String::from("let a = \"hi\";"));

    assert_eq!(tokens.len(), 7);
    assert!(tokens.get(0).unwrap().eq(&Tok::StartProgram));
    assert!(tokens.get(1).unwrap().eq(&Tok::StartStatement));
    assert!(tokens.get(2).unwrap().eq(&Tok::Let));
    assert!(tokens.get(3).unwrap().eq(&Tok::Name { name: String::from("a") }));
    assert!(tokens.get(4).unwrap().eq(&Tok::Equal));
    assert!(tokens.get(5).unwrap().eq(&Tok::String { value: String::from("hi") }));
    assert!(tokens.get(6).unwrap().eq(&Tok::Semi));
}

#[test]
fn test_let_function() {
    let file = fs::read_to_string("js/functions/let_function.js");

    let mut lex = Lexer::new();
    let tokens = lex.lex(file.unwrap());

    assert_eq!(tokens.len(), 16);
    assert!(tokens.get(0).unwrap().eq(&Tok::Let));
    assert!(tokens.get(1).unwrap().eq(&Tok::Name { name: String::from("f") }));
    assert!(tokens.get(2).unwrap().eq(&Tok::Equal));
    assert!(tokens.get(3).unwrap().eq(&Tok::Function));
    assert!(tokens.get(4).unwrap().eq(&Tok::Lpar));
    assert!(tokens.get(5).unwrap().eq(&Tok::Rpar));
    assert!(tokens.get(6).unwrap().eq(&Tok::Lbrace));
    assert!(tokens.get(7).unwrap().eq(&Tok::Name { name: String::from("console") }));
    assert!(tokens.get(8).unwrap().eq(&Tok::Dot));
    assert!(tokens.get(9).unwrap().eq(&Tok::Name { name: String::from("log") }));
    assert!(tokens.get(10).unwrap().eq(&Tok::Lpar));
    assert!(tokens.get(11).unwrap().eq(&Tok::String { value: String::from("hi") }));
    assert!(tokens.get(12).unwrap().eq(&Tok::Rpar));
    assert!(tokens.get(13).unwrap().eq(&Tok::Semi));
    assert!(tokens.get(14).unwrap().eq(&Tok::Rbrace));
    assert!(tokens.get(15).unwrap().eq(&Tok::Semi));
}

#[test]
fn test_let_function_arrow() {
    let file = fs::read_to_string("js/functions/let_arrow.js");

    let mut lex = Lexer::new();
    let tokens = lex.lex(file.unwrap());

    assert_eq!(tokens.len(), 16);
    assert!(tokens.get(0).unwrap().eq(&Tok::Let));
    assert!(tokens.get(1).unwrap().eq(&Tok::Name { name: String::from("f") }));
    assert!(tokens.get(2).unwrap().eq(&Tok::Equal));
    assert!(tokens.get(3).unwrap().eq(&Tok::Lpar));
    assert!(tokens.get(4).unwrap().eq(&Tok::Rpar));
    assert!(tokens.get(5).unwrap().eq(&Tok::RdoubleArrow));
    assert!(tokens.get(6).unwrap().eq(&Tok::Lbrace));
    assert!(tokens.get(7).unwrap().eq(&Tok::Name { name: String::from("console") }));
    assert!(tokens.get(8).unwrap().eq(&Tok::Dot));
    assert!(tokens.get(9).unwrap().eq(&Tok::Name { name: String::from("log") }));
    assert!(tokens.get(10).unwrap().eq(&Tok::Lpar));
    assert!(tokens.get(11).unwrap().eq(&Tok::String { value: String::from("hi") }));
    assert!(tokens.get(12).unwrap().eq(&Tok::Rpar));
    assert!(tokens.get(13).unwrap().eq(&Tok::Semi));
    assert!(tokens.get(14).unwrap().eq(&Tok::Rbrace));
    assert!(tokens.get(15).unwrap().eq(&Tok::Semi));
}

#[test]
fn test_const_function() {
    let file = fs::read_to_string("js/functions/const_function.js");

    let mut lex = Lexer::new();
    let tokens = lex.lex(file.unwrap());

    assert_eq!(tokens.len(), 20);
    assert!(tokens.get(0).unwrap().eq(&Tok::StartProgram));
    assert!(tokens.get(1).unwrap().eq(&Tok::StartStatement));
    assert!(tokens.get(2).unwrap().eq(&Tok::Const));
    assert!(tokens.get(3).unwrap().eq(&Tok::Name { name: String::from("f") }));
    assert!(tokens.get(4).unwrap().eq(&Tok::Equal));
    assert!(tokens.get(5).unwrap().eq(&Tok::Function));
    assert!(tokens.get(6).unwrap().eq(&Tok::Lpar));
    assert!(tokens.get(7).unwrap().eq(&Tok::Rpar));
    assert!(tokens.get(8).unwrap().eq(&Tok::Lbrace));
    assert!(tokens.get(9).unwrap().eq(&Tok::EndOfLine));
    assert!(tokens.get(10).unwrap().eq(&Tok::Name { name: String::from("console") }));
    assert!(tokens.get(11).unwrap().eq(&Tok::Dot));
    assert!(tokens.get(12).unwrap().eq(&Tok::Name { name: String::from("log") }));
    assert!(tokens.get(13).unwrap().eq(&Tok::Lpar));
    assert!(tokens.get(14).unwrap().eq(&Tok::String { value: String::from("hi") }));
    assert!(tokens.get(15).unwrap().eq(&Tok::Rpar));
    assert!(tokens.get(16).unwrap().eq(&Tok::Semi));
    assert!(tokens.get(17).unwrap().eq(&Tok::EndOfLine));
    assert!(tokens.get(18).unwrap().eq(&Tok::Rbrace));
    assert!(tokens.get(19).unwrap().eq(&Tok::Semi));
}

#[test]
fn test_const_function_arrow() {
    let file = fs::read_to_string("js/functions/const_arrow.js");

    let mut lex = Lexer::new();
    let tokens = lex.lex(file.unwrap());

    assert_eq!(tokens.len(), 20);
    assert!(tokens.get(0).unwrap().eq(&Tok::StartProgram));
    assert!(tokens.get(1).unwrap().eq(&Tok::StartStatement));
    assert!(tokens.get(2).unwrap().eq(&Tok::Const));
    assert!(tokens.get(3).unwrap().eq(&Tok::Name { name: String::from("f") }));
    assert!(tokens.get(4).unwrap().eq(&Tok::Equal));
    assert!(tokens.get(5).unwrap().eq(&Tok::Lpar));
    assert!(tokens.get(6).unwrap().eq(&Tok::Rpar));
    assert!(tokens.get(7).unwrap().eq(&Tok::RdoubleArrow));
    assert!(tokens.get(8).unwrap().eq(&Tok::Lbrace));
    assert!(tokens.get(9).unwrap().eq(&Tok::EndOfLine));
    assert!(tokens.get(10).unwrap().eq(&Tok::Name { name: String::from("console") }));
    assert!(tokens.get(11).unwrap().eq(&Tok::Dot));
    assert!(tokens.get(12).unwrap().eq(&Tok::Name { name: String::from("log") }));
    assert!(tokens.get(13).unwrap().eq(&Tok::Lpar));
    assert!(tokens.get(14).unwrap().eq(&Tok::String { value: String::from("hi") }));
    assert!(tokens.get(15).unwrap().eq(&Tok::Rpar));
    assert!(tokens.get(16).unwrap().eq(&Tok::Semi));
    assert!(tokens.get(17).unwrap().eq(&Tok::EndOfLine));
    assert!(tokens.get(18).unwrap().eq(&Tok::Rbrace));
    assert!(tokens.get(19).unwrap().eq(&Tok::Semi));
}

#[test]
fn test_function() {
    let file = fs::read_to_string("js/functions/function.js");

    let mut lex = Lexer::new();
    let tokens = lex.lex(file.unwrap());

    assert_eq!(tokens.len(), 16);
    assert!(tokens.get(0).unwrap().eq(&Tok::StartProgram));
    assert!(tokens.get(1).unwrap().eq(&Tok::Function));
    assert!(tokens.get(2).unwrap().eq(&Tok::Name { name: String::from("f") }));
    assert!(tokens.get(3).unwrap().eq(&Tok::Lpar));
    assert!(tokens.get(4).unwrap().eq(&Tok::Rpar));
    assert!(tokens.get(5).unwrap().eq(&Tok::Lbrace));
    assert!(tokens.get(6).unwrap().eq(&Tok::EndOfLine));
    assert!(tokens.get(7).unwrap().eq(&Tok::Name { name: String::from("console") }));
    assert!(tokens.get(8).unwrap().eq(&Tok::Dot));
    assert!(tokens.get(9).unwrap().eq(&Tok::Name { name: String::from("log") }));
    assert!(tokens.get(10).unwrap().eq(&Tok::Lpar));
    assert!(tokens.get(11).unwrap().eq(&Tok::String { value: String::from("hi") }));
    assert!(tokens.get(12).unwrap().eq(&Tok::Rpar));
    assert!(tokens.get(13).unwrap().eq(&Tok::Semi));
    assert!(tokens.get(14).unwrap().eq(&Tok::EndOfLine));
    assert!(tokens.get(15).unwrap().eq(&Tok::Rbrace));
}
