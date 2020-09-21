use crate::parser::symbols::JSItem;
use std::collections::HashMap;
use std::collections::hash_map::RandomState;

pub(crate) fn inherit(mut scope: HashMap<String, JSItem>, from: String, to: String) -> HashMap<String, JSItem, RandomState> {
    let mut new_prototype = HashMap::new();
    new_prototype.insert("constructor".to_string(), JSItem::ObjectReference {path: vec![to.clone()]});

    let object = scope.get(&from).unwrap();
    match object {
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
                                    let mut reference_vec = vec![from.to_string(), key.to_string()];
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
    scope.insert(to.clone(), JSItem::Object {
        mutable: false,
        properties: new_properties
    });
    return scope;
}