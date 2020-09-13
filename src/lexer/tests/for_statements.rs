use std::fs;
use crate::lexer::lexer::Lexer;
use crate::lexer::js_token::Tok;

#[test]
fn test_for() {
    let file = fs::read_to_string("js/if_while_for/for1.js");

    let mut lex = Lexer::new();
    let tokens = lex.lex(file.unwrap());

    assert_eq!(tokens.len(), 25);
    let expected = vec![
        Tok::For,
        Tok::Lpar,
        Tok::Let,
        Tok::Name {name: "i".to_string()},
        Tok::Equal,
        Tok::Float {value: 0.},
        Tok::Semi,
        Tok::Name {name: "i".to_string()},
        Tok::Less,
        Tok::Float {value: 10.},
        Tok::Semi,
        Tok::Name {name: "i".to_string()},
        Tok::PlusPlus,
        Tok::Rpar,
        Tok::Lbrace,
        Tok::EndOfLine,
        Tok::Name {name: "console".to_string()},
        Tok::Dot,
        Tok::Name {name: "log".to_string()},
        Tok::Lpar,
        Tok::Name {name: "i".to_string()},
        Tok::Rpar,
        Tok::Semi,
        Tok::EndOfLine,
        Tok::Rbrace
    ];
    assert!(tokens.eq(&expected));
}

#[test]
fn test_for_test_ident() {
    let file = fs::read_to_string("js/if_while_for/for_test_ident.js");

    let mut lex = Lexer::new();
    let tokens = lex.lex(file.unwrap());

    assert_eq!(tokens.len(), 25);
    let expected = vec![
        Tok::For,
        Tok::Lpar,
        Tok::Let,
        Tok::Name {name: "i".to_string()},
        Tok::Equal,
        Tok::Float {value: 0.},
        Tok::Semi,
        Tok::Name {name: "i".to_string()},
        Tok::Less,
        Tok::Name {name: "j".to_string()},
        Tok::Semi,
        Tok::Name {name: "i".to_string()},
        Tok::PlusPlus,
        Tok::Rpar,
        Tok::Lbrace,
        Tok::EndOfLine,
        Tok::Name {name: "console".to_string()},
        Tok::Dot,
        Tok::Name {name: "log".to_string()},
        Tok::Lpar,
        Tok::Name {name: "i".to_string()},
        Tok::Rpar,
        Tok::Semi,
        Tok::EndOfLine,
        Tok::Rbrace
    ];
    assert!(tokens.eq(&expected));
}