use crate::parser::symbols::JSItem;
use std::collections::HashMap;
use crate::vm::vm::Vm;


fn find_vm_object(vm: &Vm, path: String) -> Option<&JSItem> {
    for i in (0..vm.scopes.len()).rev() {
        let temp_key = i.to_string() + ":" + &path;
        if vm.scopes.get(i).unwrap().contains_key(&temp_key) {
            return vm.objects.get(&temp_key)
        }
    }
    return None
}

pub(crate) fn inherit(vm: &Vm, from: JSItem, to: JSItem) -> JSItem {

    let from_key_temp = match from {
        JSItem::ObjectReference { path } => {
            return path.join(":").to_string();
        }
        _ => "".to_string()
    };

    if let Some(from_item) = find_vm_object(vm, from_key_temp) {
        let mut new_prototype = HashMap::new();
        new_prototype.insert("constructor".to_string(), to.clone());


    }


    let mut new_prototype = HashMap::new();
    new_prototype.insert("constructor".to_string(), to.clone());

    let mut from_vec: Vec<String> = vec![];
    let mut from_vec_clone: Vec<String> = vec![];
    let mut from_clone: JSItem = from.clone();

    if let JSItem::ObjectReference { path } = from {
        from_vec = path;
        from_vec_clone = from_vec.clone();
    }

    let from_key =

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