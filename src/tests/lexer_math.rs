use std::fs;
use std::any::Any;
use crate::lexer::lexer::Lexer;
use crate::lexer::js_token::Tok;

#[test]
fn test_plus() {
    let file = fs::read_to_string("js/math/plus.js");

    let mut lex = Lexer::new();
    let tokens = lex.lex(file.unwrap());

    assert_eq!(tokens.len(), 25);
    assert!(tokens.get(0).unwrap().eq(&Tok::Let));
    assert!(tokens.get(1).unwrap().eq(&Tok::Name { name: String::from("a") }));
    assert!(tokens.get(2).unwrap().eq(&Tok::Equal));
    assert!(tokens.get(3).unwrap().eq(&Tok::Float { value: 1.0 }));
    assert!(tokens.get(4).unwrap().eq(&Tok::Plus));
    assert!(tokens.get(5).unwrap().eq(&Tok::Float { value: 2.0 }));
    assert!(tokens.get(6).unwrap().eq(&Tok::Semi));

    assert!(tokens.get(7).unwrap().eq(&Tok::Name { name: String::from("a") }));
    assert!(tokens.get(8).unwrap().eq(&Tok::Equal));
    assert!(tokens.get(9).unwrap().eq(&Tok::Float { value: 1.0 }));
    assert!(tokens.get(10).unwrap().eq(&Tok::Plus));
    assert!(tokens.get(11).unwrap().eq(&Tok::Float { value: 2.0 }));
    assert!(tokens.get(12).unwrap().eq(&Tok::Semi));

    assert!(tokens.get(13).unwrap().eq(&Tok::Name { name: String::from("a") }));
    assert!(tokens.get(14).unwrap().eq(&Tok::Equal));
    assert!(tokens.get(15).unwrap().eq(&Tok::Float { value: 1.0 }));
    assert!(tokens.get(16).unwrap().eq(&Tok::Plus));
    assert!(tokens.get(17).unwrap().eq(&Tok::Float { value: 2.0 }));
    assert!(tokens.get(18).unwrap().eq(&Tok::Semi));

    assert!(tokens.get(19).unwrap().eq(&Tok::Name { name: String::from("a") }));
    assert!(tokens.get(20).unwrap().eq(&Tok::Equal));
    assert!(tokens.get(21).unwrap().eq(&Tok::Float { value: 1.0 }));
    assert!(tokens.get(22).unwrap().eq(&Tok::Plus));
    assert!(tokens.get(23).unwrap().eq(&Tok::Float { value: 2.0 }));
    assert!(tokens.get(24).unwrap().eq(&Tok::Semi));
}

#[test]
fn test_minus() {
    let file = fs::read_to_string("js/math/minus.js");

    let mut lex = Lexer::new();
    let tokens = lex.lex(file.unwrap());

    assert_eq!(tokens.len(), 25);
    assert!(tokens.get(0).unwrap().eq(&Tok::Let));
    assert!(tokens.get(1).unwrap().eq(&Tok::Name { name: String::from("a") }));
    assert!(tokens.get(2).unwrap().eq(&Tok::Equal));
    assert!(tokens.get(3).unwrap().eq(&Tok::Float { value: 1.0 }));
    assert!(tokens.get(4).unwrap().eq(&Tok::Minus));
    assert!(tokens.get(5).unwrap().eq(&Tok::Float { value: 2.0 }));
    assert!(tokens.get(6).unwrap().eq(&Tok::Semi));

    assert!(tokens.get(7).unwrap().eq(&Tok::Name { name: String::from("a") }));
    assert!(tokens.get(8).unwrap().eq(&Tok::Equal));
    assert!(tokens.get(9).unwrap().eq(&Tok::Float { value: 1.0 }));
    assert!(tokens.get(10).unwrap().eq(&Tok::Minus));
    assert!(tokens.get(11).unwrap().eq(&Tok::Float { value: 2.0 }));
    assert!(tokens.get(12).unwrap().eq(&Tok::Semi));

    assert!(tokens.get(13).unwrap().eq(&Tok::Name { name: String::from("a") }));
    assert!(tokens.get(14).unwrap().eq(&Tok::Equal));
    assert!(tokens.get(15).unwrap().eq(&Tok::Float { value: 1.0 }));
    assert!(tokens.get(16).unwrap().eq(&Tok::Minus));
    assert!(tokens.get(17).unwrap().eq(&Tok::Float { value: 2.0 }));
    assert!(tokens.get(18).unwrap().eq(&Tok::Semi));

    assert!(tokens.get(19).unwrap().eq(&Tok::Name { name: String::from("a") }));
    assert!(tokens.get(20).unwrap().eq(&Tok::Equal));
    assert!(tokens.get(21).unwrap().eq(&Tok::Float { value: 1.0 }));
    assert!(tokens.get(22).unwrap().eq(&Tok::Minus));
    assert!(tokens.get(23).unwrap().eq(&Tok::Float { value: 2.0 }));
    assert!(tokens.get(24).unwrap().eq(&Tok::Semi));
}

#[test]
fn test_mul() {
    let file = fs::read_to_string("js/math/mul.js");

    let mut lex = Lexer::new();
    let tokens = lex.lex(file.unwrap());

    assert_eq!(tokens.len(), 25);
    assert!(tokens.get(0).unwrap().eq(&Tok::Let));
    assert!(tokens.get(1).unwrap().eq(&Tok::Name { name: String::from("a") }));
    assert!(tokens.get(2).unwrap().eq(&Tok::Equal));
    assert!(tokens.get(3).unwrap().eq(&Tok::Float { value: 1.0 }));
    assert!(tokens.get(4).unwrap().eq(&Tok::Star));
    assert!(tokens.get(5).unwrap().eq(&Tok::Float { value: 2.0 }));
    assert!(tokens.get(6).unwrap().eq(&Tok::Semi));

    assert!(tokens.get(7).unwrap().eq(&Tok::Name { name: String::from("a") }));
    assert!(tokens.get(8).unwrap().eq(&Tok::Equal));
    assert!(tokens.get(9).unwrap().eq(&Tok::Float { value: 1.0 }));
    assert!(tokens.get(10).unwrap().eq(&Tok::Star));
    assert!(tokens.get(11).unwrap().eq(&Tok::Float { value: 2.0 }));
    assert!(tokens.get(12).unwrap().eq(&Tok::Semi));

    assert!(tokens.get(13).unwrap().eq(&Tok::Name { name: String::from("a") }));
    assert!(tokens.get(14).unwrap().eq(&Tok::Equal));
    assert!(tokens.get(15).unwrap().eq(&Tok::Float { value: 1.0 }));
    assert!(tokens.get(16).unwrap().eq(&Tok::Star));
    assert!(tokens.get(17).unwrap().eq(&Tok::Float { value: 2.0 }));
    assert!(tokens.get(18).unwrap().eq(&Tok::Semi));

    assert!(tokens.get(19).unwrap().eq(&Tok::Name { name: String::from("a") }));
    assert!(tokens.get(20).unwrap().eq(&Tok::Equal));
    assert!(tokens.get(21).unwrap().eq(&Tok::Float { value: 1.0 }));
    assert!(tokens.get(22).unwrap().eq(&Tok::Star));
    assert!(tokens.get(23).unwrap().eq(&Tok::Float { value: 2.0 }));
    assert!(tokens.get(24).unwrap().eq(&Tok::Semi));
}

#[test]
fn test_number_plus_return_value() {
    let file = fs::read_to_string("js/math/number_plus_return_value.js");

    let mut lex = Lexer::new();
    let tokens = lex.lex(file.unwrap());

    assert_eq!(tokens.len(), 30);
    assert!(tokens.get(0).unwrap().eq(&Tok::Let));
    assert!(tokens.get(1).unwrap().eq(&Tok::Name { name: String::from("a") }));
    assert!(tokens.get(2).unwrap().eq(&Tok::Equal));
    assert!(tokens.get(3).unwrap().eq(&Tok::Float { value: 1.0 }));
    assert!(tokens.get(4).unwrap().eq(&Tok::Plus));
    assert!(tokens.get(5).unwrap().eq(&Tok::Lpar));
    assert!(tokens.get(6).unwrap().eq(&Tok::Function));
    assert!(tokens.get(7).unwrap().eq(&Tok::Lpar));
    assert!(tokens.get(8).unwrap().eq(&Tok::Rpar));
    assert!(tokens.get(9).unwrap().eq(&Tok::Lbrace));
    assert!(tokens.get(10).unwrap().eq(&Tok::Return));
    assert!(tokens.get(11).unwrap().eq(&Tok::Float {value: 1.0}));
    assert!(tokens.get(12).unwrap().eq(&Tok::Rbrace));
    assert!(tokens.get(13).unwrap().eq(&Tok::Rpar));
    assert!(tokens.get(14).unwrap().eq(&Tok::Lpar));
    assert!(tokens.get(15).unwrap().eq(&Tok::Rpar));
    assert!(tokens.get(16).unwrap().eq(&Tok::Semi));

    assert!(tokens.get(17).unwrap().eq(&Tok::Name { name: String::from("a") }));
    assert!(tokens.get(18).unwrap().eq(&Tok::Equal));
    assert!(tokens.get(19).unwrap().eq(&Tok::Float { value: 1.0 }));
    assert!(tokens.get(20).unwrap().eq(&Tok::Plus));
    assert!(tokens.get(21).unwrap().eq(&Tok::Lpar));
    assert!(tokens.get(22).unwrap().eq(&Tok::Lpar));
    assert!(tokens.get(23).unwrap().eq(&Tok::Rpar));
    assert!(tokens.get(24).unwrap().eq(&Tok::RdoubleArrow));
    assert!(tokens.get(25).unwrap().eq(&Tok::Float {value: 1.0}));
    assert!(tokens.get(26).unwrap().eq(&Tok::Rpar));
    assert!(tokens.get(27).unwrap().eq(&Tok::Lpar));
    assert!(tokens.get(28).unwrap().eq(&Tok::Rpar));
    assert!(tokens.get(29).unwrap().eq(&Tok::Semi));
}

#[test]
fn test_bitwise() {
    let file = fs::read_to_string("js/math/bitwise.js");

    let mut lex = Lexer::new();
    let tokens = lex.lex(file.unwrap());

    assert_eq!(tokens.len(), 26);
    assert!(tokens.get(0).unwrap().eq(&Tok::Let));
    assert!(tokens.get(1).unwrap().eq(&Tok::Name { name: String::from("x") }));
    assert!(tokens.get(2).unwrap().eq(&Tok::Equal));
    assert!(tokens.get(3).unwrap().eq(&Tok::Float { value: 1.0 }));
    assert!(tokens.get(4).unwrap().eq(&Tok::LeftShift));
    assert!(tokens.get(5).unwrap().eq(&Tok::Float { value: 10.0 }));
    assert!(tokens.get(6).unwrap().eq(&Tok::Semi));

    assert!(tokens.get(7).unwrap().eq(&Tok::Let));
    assert!(tokens.get(8).unwrap().eq(&Tok::Name { name: String::from("y") }));
    assert!(tokens.get(9).unwrap().eq(&Tok::Equal));
    assert!(tokens.get(10).unwrap().eq(&Tok::Float { value: 10.0 }));
    assert!(tokens.get(11).unwrap().eq(&Tok::RightShift));
    assert!(tokens.get(12).unwrap().eq(&Tok::Float { value: 2.0 }));
    assert!(tokens.get(13).unwrap().eq(&Tok::Semi));

    assert!(tokens.get(14).unwrap().eq(&Tok::Name { name: String::from("x") }));
    assert!(tokens.get(15).unwrap().eq(&Tok::RightShiftEqual));
    assert!(tokens.get(16).unwrap().eq(&Tok::Float { value: 2.0 }));
    assert!(tokens.get(17).unwrap().eq(&Tok::Semi));

    assert!(tokens.get(18).unwrap().eq(&Tok::Name { name: String::from("y") }));
    assert!(tokens.get(19).unwrap().eq(&Tok::LeftShiftEqual));
    assert!(tokens.get(20).unwrap().eq(&Tok::Float { value: 10.0 }));
    assert!(tokens.get(21).unwrap().eq(&Tok::Semi));

    assert!(tokens.get(22).unwrap().eq(&Tok::Name { name: String::from("y") }));
    assert!(tokens.get(23).unwrap().eq(&Tok::RightShiftUnsignedEqual));
    assert!(tokens.get(24).unwrap().eq(&Tok::Float { value: 2.0 }));
    assert!(tokens.get(25).unwrap().eq(&Tok::Semi));
}