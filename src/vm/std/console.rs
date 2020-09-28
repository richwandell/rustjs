use crate::parser::symbols::{JSItem, StdFun};
use std::fmt::{Display, Formatter, Result};
use std::collections::HashMap;
use crate::lexer::js_token::Tok;
use crate::vm::interpreter::Interpreter;
use crate::vm::scope::insert::set_object;

struct LogVec(Vec<JSItem>);

impl Display for LogVec {
    #[allow(unused_must_use)]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for item in &self.0 {
            write!(f, "{} ", item);
        }
        Ok(())
    }
}

pub(crate) fn std_log(params: Vec<JSItem>) {
    let log_vec = LogVec(params);
    println!("{}", log_vec);
}

pub(crate) fn create_console(mut int: Interpreter) -> Interpreter {
    let mut p = HashMap::new();
    let log = JSItem::Std {
        params: vec![Tok::Name {name: "objs".to_string()}],
        func: StdFun::ConsoleLog
    };
    p.insert("log".to_string(), log);

    if let Ok(..) = set_object(&mut int, vec!["console".to_string()], JSItem::Object {
        mutable: false,
        properties: p
    }) {
        return int;
    }

    return int;
}