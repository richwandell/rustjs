use std::collections::HashMap;
use crate::parser::symbols::{JSItem};
use crate::vm::vm::Vm;
use crate::vm::scope::insert::set_object;

pub(crate) fn create_object(mut vm: Vm) -> Vm {
    let mut object_prototype = HashMap::new();
    object_prototype.insert("constructor".to_string(), JSItem::ObjectReference {path: vec!["0:Object".to_string()]});

    let mut object_properties = HashMap::new();
    object_properties.insert("prototype".to_string(), JSItem::Object {
        mutable: false,
        properties: object_prototype
    });

    object_properties.insert("__proto__".to_string(), JSItem::ObjectReference {path: vec!["0:Object".to_string()]});

    if let Ok(..) = set_object(&mut vm, vec!["Object".to_string()], JSItem::Object {
        mutable: false,
        properties: object_properties
    }, true) {
        return vm;
    }

    return vm;
}