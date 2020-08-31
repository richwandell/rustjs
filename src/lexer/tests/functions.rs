use std::fs;
use crate::lexer::lexer::Lexer;
use crate::lexer::js_token::Tok;

#[test]
fn test_declare_string_variable() {
    let mut lex = Lexer::new();
    let tokens = lex.lex(String::from("let a = \"hi\";"));

    assert_eq!(tokens.len(), 5);
    let expected = vec![
        Tok::Let,
        Tok::Name { name: String::from("a") },
        Tok::Equal,
        Tok::String { value: String::from("hi") },
        Tok::Semi
    ];
    assert!(tokens.eq(&expected));
}

#[test]
fn test_let_function() {
    let file = fs::read_to_string("js/functions/let_function.js");

    let mut lex = Lexer::new();
    let tokens = lex.lex(file.unwrap());


    assert_eq!(tokens.len(), 18);
    let expected = vec![
        Tok::Let,
        Tok::Name { name: String::from("f") },
        Tok::Equal,
        Tok::Function,
        Tok::Lpar,
        Tok::Rpar,
        Tok::Lbrace,
        Tok::EndOfLine,
        Tok::Name { name: String::from("console") },
        Tok::Dot,
        Tok::Name { name: String::from("log") },
        Tok::Lpar,
        Tok::String { value: String::from("hi") },
        Tok::Rpar,
        Tok::Semi,
        Tok::EndOfLine,
        Tok::Rbrace,
        Tok::Semi,
    ];
    assert!(tokens.eq(&expected));
}

#[test]
fn test_let_function_arrow() {
    let file = fs::read_to_string("js/functions/let_arrow.js");

    let mut lex = Lexer::new();
    let tokens = lex.lex(file.unwrap());

    assert_eq!(tokens.len(), 18);
    let expected = vec![
        Tok::Let,
        Tok::Name { name: String::from("f") },
        Tok::Equal,
        Tok::Lpar,
        Tok::Rpar,
        Tok::RdoubleArrow,
        Tok::Lbrace,
        Tok::EndOfLine,
        Tok::Name { name: String::from("console") },
        Tok::Dot,
        Tok::Name { name: String::from("log") },
        Tok::Lpar,
        Tok::String { value: String::from("hi") },
        Tok::Rpar,
        Tok::Semi,
        Tok::EndOfLine,
        Tok::Rbrace,
        Tok::Semi
    ];
    assert!(tokens.eq(&expected));
}

#[test]
fn test_const_function() {
    let file = fs::read_to_string("js/functions/const_function.js");

    let mut lex = Lexer::new();
    let tokens = lex.lex(file.unwrap());

    assert_eq!(tokens.len(), 18);
    let expected = vec![
        Tok::Const,
        Tok::Name { name: String::from("f") },
        Tok::Equal,
        Tok::Function,
        Tok::Lpar,
        Tok::Rpar,
        Tok::Lbrace,
        Tok::EndOfLine,
        Tok::Name { name: String::from("console") },
        Tok::Dot,
        Tok::Name { name: String::from("log") },
        Tok::Lpar,
        Tok::String { value: String::from("hi") },
        Tok::Rpar,
        Tok::Semi,
        Tok::EndOfLine,
        Tok::Rbrace,
        Tok::Semi
    ];
    assert!(tokens.eq(&expected));
}

#[test]
fn test_const_function_arrow() {
    let file = fs::read_to_string("js/functions/const_arrow.js");

    let mut lex = Lexer::new();
    let tokens = lex.lex(file.unwrap());

    assert_eq!(tokens.len(), 18);
    let expected = vec![
        Tok::Const,
        Tok::Name { name: String::from("f") },
        Tok::Equal,
        Tok::Lpar,
        Tok::Rpar,
        Tok::RdoubleArrow,
        Tok::Lbrace,
        Tok::EndOfLine,
        Tok::Name { name: String::from("console") },
        Tok::Dot,
        Tok::Name { name: String::from("log") },
        Tok::Lpar,
        Tok::String { value: String::from("hi") },
        Tok::Rpar,
        Tok::Semi,
        Tok::EndOfLine,
        Tok::Rbrace,
        Tok::Semi
    ];
    assert!(tokens.eq(&expected));
}

#[test]
fn test_function() {
    let file = fs::read_to_string("js/functions/function.js");

    let mut lex = Lexer::new();
    let tokens = lex.lex(file.unwrap());

    assert_eq!(tokens.len(), 15);
    let expected = vec![
        Tok::Function,
        Tok::Name { name: String::from("f") },
        Tok::Lpar,
        Tok::Rpar,
        Tok::Lbrace,
        Tok::EndOfLine,
        Tok::Name { name: String::from("console") },
        Tok::Dot,
        Tok::Name { name: String::from("log") },
        Tok::Lpar,
        Tok::String { value: String::from("hi") },
        Tok::Rpar,
        Tok::Semi,
        Tok::EndOfLine,
        Tok::Rbrace
    ];
    assert!(tokens.eq(&expected));
}
