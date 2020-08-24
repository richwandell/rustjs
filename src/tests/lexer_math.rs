use std::fs;
use std::any::Any;
use crate::lexer::lexer::Lexer;
use crate::lexer::js_token::Tok;

#[test]
fn test_plus() {
    let file = fs::read_to_string("js/math/plus.js");

    let mut lex = Lexer::new();
    let tokens = lex.lex(file.unwrap());

    assert_eq!(tokens.len(), 30);
    let expected = vec![
        Tok::StartProgram,
        Tok::StartStatement,
        Tok::Let,
        Tok::Name { name: String::from("a") },
        Tok::Equal,
        Tok::Float { value: 1.0 },
        Tok::Plus,
        Tok::Float { value: 2.0 },
        Tok::Semi,
        Tok::EndOfLine,
        Tok::Name { name: String::from("a") },
        Tok::Equal,
        Tok::Float { value: 1.0 },
        Tok::Plus,
        Tok::Float { value: 2.0 },
        Tok::Semi,
        Tok::EndOfLine,
        Tok::Name { name: String::from("a") },
        Tok::Equal,
        Tok::Float { value: 1.0 },
        Tok::Plus,
        Tok::Float { value: 2.0 },
        Tok::Semi,
        Tok::EndOfLine,
        Tok::Name { name: String::from("a") },
        Tok::Equal,
        Tok::Float { value: 1.0 },
        Tok::Plus,
        Tok::Float { value: 2.0 },
        Tok::Semi
    ];
    assert!(tokens.eq(&expected));
}

#[test]
fn test_minus() {
    let file = fs::read_to_string("js/math/minus.js");

    let mut lex = Lexer::new();
    let tokens = lex.lex(file.unwrap());

    assert_eq!(tokens.len(), 30);
    let expected = vec![
        Tok::StartProgram,
        Tok::StartStatement,
        Tok::Let,
        Tok::Name { name: String::from("a") },
        Tok::Equal,
        Tok::Float { value: 1.0 },
        Tok::Minus,
        Tok::Float { value: 2.0 },
        Tok::Semi,
        Tok::EndOfLine,
        Tok::Name { name: String::from("a") },
        Tok::Equal,
        Tok::Float { value: 1.0 },
        Tok::Minus,
        Tok::Float { value: 2.0 },
        Tok::Semi,
        Tok::EndOfLine,
        Tok::Name { name: String::from("a") },
        Tok::Equal,
        Tok::Float { value: 1.0 },
        Tok::Minus,
        Tok::Float { value: 2.0 },
        Tok::Semi,
        Tok::EndOfLine,
        Tok::Name { name: String::from("a") },
        Tok::Equal,
        Tok::Float { value: 1.0 },
        Tok::Minus,
        Tok::Float { value: 2.0 },
        Tok::Semi
    ];
    assert!(tokens.eq(&expected));
}

#[test]
fn test_mul() {
    let file = fs::read_to_string("js/math/mul.js");

    let mut lex = Lexer::new();
    let tokens = lex.lex(file.unwrap());

    assert_eq!(tokens.len(), 30);
    let expected = vec![
        Tok::StartProgram,
        Tok::StartStatement,
        Tok::Let,
        Tok::Name { name: String::from("a") },
        Tok::Equal,
        Tok::Float { value: 1.0 },
        Tok::Star,
        Tok::Float { value: 2.0 },
        Tok::Semi,
        Tok::EndOfLine,
        Tok::Name { name: String::from("a") },
        Tok::Equal,
        Tok::Float { value: 1.0 },
        Tok::Star,
        Tok::Float { value: 2.0 },
        Tok::Semi,
        Tok::EndOfLine,
        Tok::Name { name: String::from("a") },
        Tok::Equal,
        Tok::Float { value: 1.0 },
        Tok::Star,
        Tok::Float { value: 2.0 },
        Tok::Semi,
        Tok::EndOfLine,
        Tok::Name { name: String::from("a") },
        Tok::Equal,
        Tok::Float { value: 1.0 },
        Tok::Star,
        Tok::Float { value: 2.0 },
        Tok::Semi
    ];
    assert!(tokens.eq(&expected));
}

#[test]
fn test_number_plus_return_value() {
    let file = fs::read_to_string("js/math/number_plus_return_value.js");

    let mut lex = Lexer::new();
    let tokens = lex.lex(file.unwrap());

    assert_eq!(tokens.len(), 33);
    let expected = vec![
        Tok::StartProgram,
        Tok::StartStatement,
        Tok::Let,
        Tok::Name { name: String::from("a") },
        Tok::Equal,
        Tok::Float { value: 1.0 },
        Tok::Plus,
        Tok::Lpar,
        Tok::Function,
        Tok::Lpar,
        Tok::Rpar,
        Tok::Lbrace,
        Tok::Return,
        Tok::Float { value: 1.0 },
        Tok::Rbrace,
        Tok::Rpar,
        Tok::Lpar,
        Tok::Rpar,
        Tok::Semi,
        Tok::EndOfLine,
        Tok::Name { name: String::from("a") },
        Tok::Equal,
        Tok::Float { value: 1.0 },
        Tok::Plus,
        Tok::Lpar,
        Tok::Lpar,
        Tok::Rpar,
        Tok::RdoubleArrow,
        Tok::Float { value: 1.0 },
        Tok::Rpar,
        Tok::Lpar,
        Tok::Rpar,
        Tok::Semi
    ];
    assert!(tokens.eq(&expected));
}

#[test]
fn test_bitwise() {
    let file = fs::read_to_string("js/math/bitwise.js");

    let mut lex = Lexer::new();
    let tokens = lex.lex(file.unwrap());

    assert_eq!(tokens.len(), 33);
    let expected = vec![
        Tok::StartProgram,
        Tok::StartStatement,
        Tok::Let,
        Tok::Name { name: String::from("x") },
        Tok::Equal,
        Tok::Float { value: 1.0 },
        Tok::LeftShift,
        Tok::Float { value: 10.0 },
        Tok::Semi,
        Tok::EndOfLine,
        Tok::StartStatement,
        Tok::Let,
        Tok::Name { name: String::from("y") },
        Tok::Equal,
        Tok::Float { value: 10.0 },
        Tok::RightShift,
        Tok::Float { value: 2.0 },
        Tok::Semi,
        Tok::EndOfLine,
        Tok::Name { name: String::from("x") },
        Tok::RightShiftEqual,
        Tok::Float { value: 2.0 },
        Tok::Semi,
        Tok::EndOfLine,
        Tok::Name { name: String::from("y") },
        Tok::LeftShiftEqual,
        Tok::Float { value: 10.0 },
        Tok::Semi,
        Tok::EndOfLine,
        Tok::Name { name: String::from("y") },
        Tok::RightShiftUnsignedEqual,
        Tok::Float { value: 2.0 },
        Tok::Semi
    ];

    assert!(tokens.eq(&expected));
}