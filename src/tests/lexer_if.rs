use std::fs;
use std::any::Any;
use crate::lexer::lexer::Lexer;
use crate::lexer::js_token::Tok;

#[test]
fn test_simple_if() {
    let file = fs::read_to_string("js/if_while_for/if1.js");

    let mut lex = Lexer::new();
    let tokens = lex.lex(file.unwrap());

    assert_eq!(tokens.len(), 25);
    assert!(tokens.get(0).unwrap().eq(&Tok::StartProgram));
    assert!(tokens.get(1).unwrap().eq(&Tok::StartStatement));
    assert!(tokens.get(2).unwrap().eq(&Tok::Const));
    assert!(tokens.get(3).unwrap().eq(&Tok::Name { name: String::from("x") }));
    assert!(tokens.get(4).unwrap().eq(&Tok::Equal));
    assert!(tokens.get(5).unwrap().eq(&Tok::Float {value: 5.}));
    assert!(tokens.get(6).unwrap().eq(&Tok::Semi));
    assert!(tokens.get(7).unwrap().eq(&Tok::EndOfLine));
    assert!(tokens.get(8).unwrap().eq(&Tok::If));
    assert!(tokens.get(9).unwrap().eq(&Tok::Lpar));
    assert!(tokens.get(10).unwrap().eq(&Tok::Name {name: String::from("x")}));
    assert!(tokens.get(11).unwrap().eq(&Tok::Greater));
    assert!(tokens.get(12).unwrap().eq(&Tok::Float {value: 2.}));
    assert!(tokens.get(13).unwrap().eq(&Tok::Rpar));
    assert!(tokens.get(14).unwrap().eq(&Tok::Lbrace));
    assert!(tokens.get(15).unwrap().eq(&Tok::EndOfLine));
    assert!(tokens.get(16).unwrap().eq(&Tok::Name { name: String::from("console") }));
    assert!(tokens.get(17).unwrap().eq(&Tok::Dot));
    assert!(tokens.get(18).unwrap().eq(&Tok::Name { name: String::from("log") }));
    assert!(tokens.get(19).unwrap().eq(&Tok::Lpar));
    assert!(tokens.get(20).unwrap().eq(&Tok::String { value: String::from("hi") }));
    assert!(tokens.get(21).unwrap().eq(&Tok::Rpar));
    assert!(tokens.get(22).unwrap().eq(&Tok::Semi));
    assert!(tokens.get(23).unwrap().eq(&Tok::EndOfLine));
    assert!(tokens.get(24).unwrap().eq(&Tok::Rbrace));
}