use std::fs;
use crate::lexer::lexer::Lexer;
use crate::lexer::js_token::Tok;
use crate::lexer::line_char_iterator::LineCharIterator;

#[test]
fn test_line_char_iterator() {
    let mut lci = LineCharIterator::new("js/some-data.csv");

    let mut forward = vec![];
    loop {
        if let Some(item) = lci.next() {
            forward.push(item);
        } else {
            break;
        }
    }

    let mut fi = forward.len();
    loop {
        if let Some(item) = lci.prev() {
            fi -= 1;
            assert_eq!(&item, forward.get(fi).unwrap());
        } else {
            break;
        }
    }
}