use crate::parser::symbols::{JSItem, StdFun};
use std::collections::HashMap;
use crate::lexer::js_token::Tok;
use std::fmt::{Display, Formatter, Result};
use crate::vm::js_output::JSOutput;

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

pub(crate) fn create_console() -> JSItem {
    let mut p = HashMap::new();
    let log = JSItem::Std {
        params: vec![Tok::Name {name: "item".to_string()}],
        func: StdFun::ConsoleLog
    };
    p.insert("log".to_string(), log);
    JSItem::Object {
        mutable: false,
        properties: p
    }
}

pub(crate) fn create_std_objects() -> HashMap<String, JSItem> {
    let mut f = HashMap::new();
    f.insert("console".to_string(), create_console());
    return f;
}