use crate::lexer::string_iterator::StringIterator;

mod functions;
mod if_statement;
mod math;
mod one_liners;

#[test]
fn test_iterator() {
    let s = "let a = \"hi\";".chars();
    let mut it: StringIterator = StringIterator::new(s);

    loop {
        let ch = it.next();
        match ch {
            Err(_) => {
                break
            }
            Ok(ch) => {
                if ch == 't' {
                    let ch = it.prev().unwrap_or('f');
                    assert_eq!(ch, 't');
                    assert_eq!(2, it.index());
                    break
                }
            }
        }
    }
}