use std::fs;
use crate::lexer::lexer::Lexer;
use crate::lexer::js_token::Tok;

#[test]
fn test_simple_if() {
    let file = fs::read_to_string("js/if_while_for/if1.js");

    let mut lex = Lexer::new();
    let tokens = lex.lex(file.unwrap());

    assert_eq!(tokens.len(), 23);
    let expected = vec![
        Tok::Const,
        Tok::Name { name: String::from("x") },
        Tok::Equal,
        Tok::Float { value: 5. },
        Tok::Semi,
        Tok::EndOfLine,
        Tok::If,
        Tok::Lpar,
        Tok::Name { name: String::from("x") },
        Tok::Greater,
        Tok::Float { value: 2. },
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