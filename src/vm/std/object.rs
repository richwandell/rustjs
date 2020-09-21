use std::collections::HashMap;
use crate::parser::symbols::{JSItem, StdFun};
use crate::lexer::js_token::Tok;

pub(crate) fn create_object(mut scope: HashMap<String, JSItem>) -> HashMap<String, JSItem> {
    let mut object_prototype = HashMap::new();
    object_prototype.insert("constructor".to_string(), JSItem::ObjectReference {path: vec!["Object".to_string()]});

    let mut object_properties = HashMap::new();
    object_properties.insert("prototype".to_string(), JSItem::Object {
        mutable: false,
        properties: object_prototype
    });

    object_properties.insert("keys".to_string(), JSItem::Std {
        params: vec![Tok::Name { name: "object".to_string() }],
        func: StdFun::ObjectKeys
    });

    scope.insert("Object".to_string(), JSItem::Object {
        mutable: false,
        properties: object_properties
    });
    return scope;
}