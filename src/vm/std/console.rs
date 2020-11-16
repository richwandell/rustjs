use crate::parser::symbols::{JSItem, StdFun};
use std::fmt::{Display, Formatter, Result};
use crate::lexer::js_token::Tok;
use crate::vm::vm::Vm;
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

pub(crate) fn create_console(mut vm: Vm) -> Vm {
    if let Ok(..) = set_object(&mut vm, vec!["console".to_string(), "log".to_string()], JSItem::Std {
        params: vec![Tok::Name {name: "objs".to_string()}],
        func: StdFun::ConsoleLog
    }, true) {
        if let Ok(..) = set_object(&mut vm, vec!["console".to_string()], JSItem::Object {
            mutable: false,
            properties: hashmap![
                "log".to_string() => JSItem::ObjectReference {path: vec!["0:console:log".to_string()]}
            ]
        }, true) {
            return vm;
        }
    }
    return vm;
}