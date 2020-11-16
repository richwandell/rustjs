use crate::parser::symbols::{JSItem};
use crate::vm::vm::Vm;

pub(crate) enum ObjecResult {
    Success,
    Error,
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