use std::path::Path;
use std::fs::File;
use std::error::Error;
use std::io::{BufReader, BufRead};


pub(crate) struct LineCharIterator {
    line_num: usize,
    char_num: usize,
    lines: Vec<Vec<char>>
}

impl LineCharIterator {

    pub(crate) fn new(file_name: &str) -> LineCharIterator {
        let path = Path::new(file_name);
        let display = path.display();

        // Open the path in read-only mode, returns `io::Result<File>`
        let file = match File::open(&path) {
            // The `description` method of `io::Error` returns a string that describes the error
            Err(why) => panic!("couldn't open {}: {}", display, Error::to_string(&why)),
            Ok(file) => file,
        };

        let reader = BufReader::new(file);
        let mut lines = vec![];

        for line in reader.lines() {
            if let Ok(string) = line {
                let mut chars = string.chars().collect::<Vec<char>>();
                chars.push('\n');
                lines.push(chars);
            }
        }

        LineCharIterator{
            lines,
            line_num: 0,
            char_num: 0
        }
    }

    pub(crate) fn prev(&mut self) -> Option<(usize, usize, char)> {
        if self.char_num != 0 {
            self.char_num -= 1;
            return Some((self.line_num, self.char_num, self.lines[self.line_num][self.char_num]));
        } else if self.line_num != 0 {
            self.line_num -= 1;
            self.char_num = self.lines[self.line_num].len() - 1;
            return Some((self.line_num, self.char_num, self.lines[self.line_num][self.char_num]));
        } else {
            return None;
        }
    }
}

impl Iterator for LineCharIterator {
    type Item = (usize, usize, char);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.lines[self.line_num].len() > self.char_num {
                let item = (self.line_num, self.char_num, self.lines[self.line_num][self.char_num]);
                self.char_num += 1;
                return Some(item);
            } else if self.lines.len() > self.line_num + 1 {
                self.char_num = 0;
                self.line_num += 1;
                if self.lines[self.line_num].len() > 0 {
                    let item = (self.line_num, self.char_num, self.lines[self.line_num][self.char_num]);
                    self.char_num += 1;
                    return Some(item);
                } else {
                    self.line_num += 1;
                    continue;
                }
            } else {
                return None;
            }
        }
    }
}