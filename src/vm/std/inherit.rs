use crate::parser::symbols::JSItem;
use std::collections::HashMap;
use crate::vm::interpreter::Interpreter;
use crate::vm::helpers::{find_object_from_reference};

pub(crate) fn inherit(int: &Interpreter, from: JSItem, to: JSItem) -> JSItem {

    let mut new_prototype = HashMap::new();
    new_prototype.insert("constructor".to_string(), to.clone());

    let mut from_vec: Vec<String> = vec![];
    let mut from_vec_clone: Vec<String> = vec![];
    let mut from_clone: JSItem = from.clone();

    if let JSItem::ObjectReference { path } = from {
        from_vec = path;
        from_vec_clone = from_vec.clone();
    }

    if let Ok(object_ref) = find_object_from_reference(&int, from_vec) {
        match object_ref {
            JSItem::Object { mutable: _, properties } => {
                let prototype = properties.get("prototype").unwrap();
                match prototype {
                    JSItem::Object { mutable:_, properties } => {
                        let keys = properties.keys();
                        for key in keys {
                            if !key.eq(&String::from("constructor")) {
                                let item = properties.get(key).unwrap();
                                match item {
                                    JSItem::ObjectReference { path } => {
                                        new_prototype.insert(key.clone(), JSItem::ObjectReference {
                                            path: path.to_vec()
                                        });
                                    }
                                    JSItem::Std { params:_, func:_ } => {
                                        let mut reference_vec = from_vec_clone.clone();
                                        reference_vec.push(key.to_string());

                                        new_prototype.insert(key.clone(), JSItem::ObjectReference {
                                            path: reference_vec
                                        });
                                    }
                                    JSItem::Function { mutable:_, params:_, properties:_, body:_ } => {
                                        let mut reference_vec = from_vec_clone.clone();
                                        reference_vec.push(key.to_string());

                                        new_prototype.insert(key.clone(), JSItem::ObjectReference {
                                            path: reference_vec
                                        });
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        let mut new_properties = HashMap::new();
        new_properties.insert("prototype".to_string(), JSItem::Object {
            mutable: false,
            properties: new_prototype
        });

        new_properties.insert("__proto__".to_string(), from_clone);

        return JSItem::Object {
            mutable: false,
            properties: new_properties
        };
    }
    return JSItem::Undefined
}