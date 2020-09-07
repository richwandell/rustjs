use std::str::Chars;

pub(crate) struct StringIterator {
    chars: Vec<char>,
    current_index: i64
}

pub(crate) enum IteratorEnd {
    End
}

impl StringIterator {
    pub(crate) fn new(chars: Chars) -> StringIterator {
        let chars: Vec<char> = chars.collect();
        StringIterator{
            chars,
            current_index: 0
        }
    }

    pub(crate) fn next(&mut self) -> Result<char, IteratorEnd> {
        if self.chars.len() - 1 >= self.current_index as usize {
            let item = self.chars[self.current_index as usize];
            self.current_index += 1;
            return Ok(item);
        }
        Err(IteratorEnd::End)
    }

    pub(crate) fn prev(&mut self) -> Result<char, IteratorEnd> {
        if self.chars.len() - 1 >= (self.current_index - 1) as usize {
            self.current_index -= 1;
            let item = self.chars[self.current_index as usize];
            return Ok(item);
        }
        Err(IteratorEnd::End)
    }

    #[allow(dead_code)]
    pub(crate) fn len(&mut self) -> i64 {
        self.chars.len() as i64
    }

    #[allow(dead_code)]
    pub(crate) fn index(&mut self) -> i64 {
        self.current_index
    }
}

