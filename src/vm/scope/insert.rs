use crate::parser::symbols::{JSItem};
use crate::vm::vm::Vm;
use std::collections::HashMap;

pub(crate) enum ObjecResult {
    Success,
    Error,
}

pub(crate) fn add_to_located_obj(vm: &mut Vm, scope_num: usize, location: String, value: JSItem, name: String) -> Vec<String> {
    let mut path_dumb = location.split(":").collect::<Vec<&str>>();
    path_dumb.remove(0);

    let mut path = vec![];
    for item in path_dumb {
        path.push(item.to_string())
    }
    path.push(name.clone());
    let mut scope = vm.scopes.get_mut(scope_num).unwrap();
    let scoped_item_key = location + ":" + &name;
    scope.insert(path.join(":"), scoped_item_key.clone());

    vm.objects.insert(scoped_item_key.clone(), JSItem::Located {
        scope: scope_num,
        location: scoped_item_key.clone(),
        object: Box::new(value),
    });
    path.insert(0, scope_num.to_string());
    return path;
}

pub(crate) fn locate_obj_props(vm: &mut Vm, name: String, obj: JSItem) -> JSItem {
    match obj {
        JSItem::Object { mutable, mut properties } => {
            let mut keys = vec![];

            for key in properties.keys() {
                keys.push(key.clone());
            }

            for key in keys {
                if let JSItem::ObjectReference { mut path } = properties.remove(&key.clone()).unwrap() {
                    let path_key = path.join(":");
                    let scope_end = vm.scopes.len() - 1;
                    if let Some(real_path) = vm.scopes.get_mut(scope_end).unwrap().remove(&path_key) {
                        if let JSItem::Located { scope, location, object } = vm.objects.remove(&real_path).unwrap() {
                            path.remove(0);
                            path.insert(0, name.clone());
                            path.insert(0, scope_end.to_string());
                            vm.objects.insert(path.join(":"), JSItem::Located {
                                scope,
                                location: path.join(":"),
                                object
                            });

                            properties.insert(key, JSItem::ObjectReference { path });
                        }
                    }
                }
            }
            JSItem::Object {mutable, properties}
        }
        _ => JSItem::Null
    }
}

pub(crate) fn set_object(vm: &mut Vm, mut path: Vec<String>, obj: JSItem, overwrite: bool) -> Result<ObjecResult, ObjecResult> {
    let scope_num = vm.scopes.len() - 1;
    let mut scope = vm.scopes.get_mut(scope_num).unwrap();

    let scoped_item_key = scope_num.to_string() + ":" + &path.join(":");
    if !overwrite && scope.contains_key(&scoped_item_key) {
        return Err(ObjecResult::Error);
    }

    vm.objects.insert(scoped_item_key.clone(), JSItem::Located {
        scope: scope_num,
        location: scoped_item_key.clone(),
        object: Box::new(obj),
    });

    scope.insert(path.join(":"), scoped_item_key.clone());
    return Ok(ObjecResult::Success);
}

pub(crate) fn load_object(vm: &mut Vm, mut path: Vec<String>) -> Result<ObjecResult, ObjecResult> {
    let path_key = path.join(":");
    for i in (0..vm.scopes.len()).rev() {
        if vm.scopes.get(i).unwrap().contains_key(&path_key) {
            let object_key = vm.scopes.get(i).unwrap().get(&path_key).unwrap();
            let item = vm.objects.remove(object_key).unwrap();
            vm.stack.push(item);
            return Ok(ObjecResult::Success);
        }
    }
    return Err(ObjecResult::Error);
}

pub(crate) fn load_prop(vm: &mut Vm, prop: String) -> Result<ObjecResult, ObjecResult> {
    if vm.stack.len() == 0 {
        return Err(ObjecResult::Error);
    }
    let item = vm.stack.pop().unwrap();
    match item {
        JSItem::Located { scope, location, object } => {
            match *object {
                JSItem::Object { mutable, properties } => {
                    if let Some(item) = properties.get(&prop) {
                        match item {
                            JSItem::ObjectReference { path } => {
                                let new_item_key = path.join(":");
                                let item1 = vm.objects.remove(&new_item_key).unwrap();
                                vm.stack.push(item1);
                                vm.objects.insert(location.clone(), JSItem::Located {
                                    scope,
                                    location,
                                    object: Box::from(JSItem::Object { mutable, properties })
                                });
                                return Ok(ObjecResult::Success);
                            }
                            _ => {
                                return Err(ObjecResult::Error);
                            }
                        }
                    }
                }
                _ => {return Err(ObjecResult::Error);}
            }
        }
        _ => {return Err(ObjecResult::Error);}
    }
    return Ok(ObjecResult::Success);
}