use std::collections::HashMap;
use crate::parser::symbols::{JSItem};
use crate::ast_interpreter::interpreter::Interpreter;
use crate::ast_interpreter::scope::insert::set_object;

pub(crate) fn create_object(mut int: Interpreter) -> Interpreter {
    let mut object_prototype = HashMap::new();
    object_prototype.insert("constructor".to_string(), JSItem::ObjectReference {path: vec!["Object".to_string()]});

    let mut object_properties = HashMap::new();
    object_properties.insert("prototype".to_string(), JSItem::Object {
        mutable: false,
        properties: object_prototype
    });

    object_properties.insert("__proto__".to_string(), JSItem::ObjectReference {path: vec!["Object".to_string()]});

    if let Ok(..) = set_object(&mut int, vec!["Object".to_string()], JSItem::Object {
        mutable: false,
        properties: object_properties
    }) {
        return int;
    }

    return int;
}