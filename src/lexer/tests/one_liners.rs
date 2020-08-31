use std::fs;
use crate::lexer::lexer::Lexer;
use crate::lexer::js_token::Tok;

#[test]
fn test_one_liner() {
    let file = fs::read_to_string("js/one_line1.js");

    let mut lex = Lexer::new();
    let tokens = lex.lex(file.unwrap());

    assert_eq!(tokens.len(), 18);
    let expected = vec![
        Tok::Name { name: String::from("tips") },
        Tok::Dot,
        Tok::Name { name: String::from("forEach") },
        Tok::Lpar,
        Tok::Lpar,
        Tok::Name { name: String::from("tip") },
        Tok::Comma,
        Tok::Name { name: String::from("i") },
        Tok::Rpar,
        Tok::RdoubleArrow,
        Tok::Name { name: String::from("console") },
        Tok::Dot,
        Tok::Name { name: String::from("log") },
        Tok::Lpar,
        Tok::Name { name: String::from("tip") },
        Tok::Rpar,
        Tok::Rpar,
        Tok::Semi
    ];

    assert!(tokens.eq(&expected));
}