use std::fs;
use crate::lexer::lexer::Lexer;
use crate::lexer::js_token::Tok;

#[test]
fn test_number_array() {
    let file = fs::read_to_string("js/arrays/number_array.js");

    let mut lex = Lexer::new();
    let tokens = lex.lex(file.unwrap());

    assert_eq!(tokens.len(), 30);
    let expected = vec![
        Tok::Let,
        Tok::Name {name: "a".to_string()},
        Tok::Equal,
        Tok::Lsqb,
        Tok::EndOfLine,
        Tok::Float {value: 1.},
        Tok::Comma,
        Tok::Float {value: 2.},
        Tok::Comma,
        Tok::Float {value: 3.},
        Tok::Comma,
        Tok::Float {value: 4.},
        Tok::Comma,
        Tok::Float {value: 5.},
        Tok::Comma,
        Tok::Float {value: 6.},
        Tok::Comma,
        Tok::EndOfLine,
        Tok::Float {value: 7.},
        Tok::Comma,
        Tok::Float {value: 8.},
        Tok::Comma,
        Tok::Float {value: 9.},
        Tok::Comma,
        Tok::Float {value: 10.},
        Tok::Comma,
        Tok::Float {value: 11.},
        Tok::EndOfLine,
        Tok::Rsqb,
        Tok::Semi
    ];
    assert!(tokens.eq(&expected));
}

#[test]
fn test_var_array() {
    let file = fs::read_to_string("js/arrays/variable_array.js");

    let mut lex = Lexer::new();
    let tokens = lex.lex(file.unwrap());

    assert_eq!(tokens.len(), 30);
    let expected = vec![
        Tok::Let,
        Tok::Name {name: "a".to_string()},
        Tok::Equal,
        Tok::Lsqb,
        Tok::EndOfLine,
        Tok::Name {name: "b".to_string()},
        Tok::Comma,
        Tok::Name {name: "c".to_string()},
        Tok::Comma,
        Tok::Name {name: "d".to_string()},
        Tok::Comma,
        Tok::Name {name: "e".to_string()},
        Tok::Comma,
        Tok::Name {name: "f".to_string()},
        Tok::Comma,
        Tok::Name {name: "g".to_string()},
        Tok::Comma,
        Tok::EndOfLine,
        Tok::Name {name: "h".to_string()},
        Tok::Comma,
        Tok::Name {name: "i".to_string()},
        Tok::Comma,
        Tok::Name {name: "j".to_string()},
        Tok::Comma,
        Tok::Name {name: "k".to_string()},
        Tok::Comma,
        Tok::Name {name: "l".to_string()},
        Tok::EndOfLine,
        Tok::Rsqb,
        Tok::Semi
    ];
    assert!(tokens.eq(&expected));
}