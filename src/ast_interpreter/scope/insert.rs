use std::collections::HashMap;
use crate::parser::symbols::{JSItem, Expression};
use crate::ast_interpreter::interpreter::Interpreter;

pub(crate) enum InsertResult {
    Ref {
        item: Vec<String>
    },
    Success,
}

fn insert_o_r_o(object: &mut HashMap<String, JSItem>, mut path: Vec<String>, new_item: JSItem) -> Result<InsertResult, ()> {
    path.reverse();


    let key = path.pop().unwrap();
    let item = object.get_mut(&key);
    match item {
        Some(i) => {
            match i {
                JSItem::Object { mutable: _, properties } => {
                    let mut new_path = path.clone();
                    new_path.push(key.clone());
                    new_path.reverse();

                    if path.len() == 0 {
                        properties.insert(key.clone(), new_item);
                        return Ok(InsertResult::Success);
                    }

                    return insert_o_r_o(properties, new_path, new_item);
                }
                _ => {
                    return Err(());
                }
            }
        }
        None => {
            let prototype = object.get_mut("prototype");
            match prototype {
                None => {
                    return Err(())
                }
                Some(i) => {
                    match i {
                        JSItem::Object { mutable: _, properties } => {
                            let mut new_path = path.clone();
                            new_path.push(key);
                            new_path.reverse();

                            return insert_o_r_o(properties, new_path, new_item);
                        }
                        _ => {
                            return Err(());
                        }
                    }
                }
            }
        }
    }
}

fn insert_o_r(scopes: &mut Vec<HashMap<String, JSItem>>, mut path: Vec<String>, new_item: JSItem) -> Result<InsertResult, ()> {
    let mut key = path.get(0).unwrap().clone();
    path.reverse();

    let mut current_scope = scopes.len() - 1;
    for i in (0..=scopes.len()-1).rev() {
        if scopes.get(i).unwrap().contains_key(&key) {
            current_scope = i;
            break;
        }
    }

    key = path.pop().unwrap();

    let mut hashmap = scopes.get_mut(current_scope).unwrap();
    let mut item = hashmap.get_mut(&key);

    if path.len() == 0 {
        hashmap.insert(key, new_item);
        return Ok(InsertResult::Success);
    }

    match item {
        Some(i) => {
            match i {
                JSItem::Variable { mutable:_, value } => {
                    match value {
                        Expression::Object { mutable:_, properties } => {
                            if path.len() == 1 {
                                properties.insert(path.pop().unwrap(), new_item);
                                return Ok(InsertResult::Success);
                            }

                            let mut new_path = path.clone();
                            new_path.push(key);
                            new_path.reverse();

                            let out = insert_o_r_o(properties, new_path, new_item);
                            match out {
                                Ok(result) => {
                                    match result {
                                        InsertResult::Ref { item:_ } => {
                                            // path = item
                                        }
                                        InsertResult::Success => {
                                            return Ok(InsertResult::Success);
                                        }
                                    }
                                }
                                Err(..) => return Err(())
                            }
                        }
                        _ => {
                            return Err(())
                        }
                    }
                }
                JSItem::ObjectReference { path: new_path } => {
                    return Ok(InsertResult::Ref { item: new_path.clone() });
                }
                JSItem::Object { mutable: _, properties } => {
                    if path.len() == 0 {
                        hashmap.insert(key, new_item);
                        return Ok(InsertResult::Success);
                    }

                    let mut new_path = path.clone();
                    new_path.push(key);
                    new_path.reverse();

                    let out = insert_o_r_o(properties, new_path, new_item);
                    match out {
                        Ok(result) => {
                            match result {
                                InsertResult::Ref { item:_ } => {
                                    // path = item
                                }
                                InsertResult::Success => {
                                    return Ok(InsertResult::Success);
                                }
                            }
                        }
                        Err(..) => return Err(())
                    }
                }
                _ => {
                    return Err(());
                }
            }
        }
        None => {
            hashmap.insert(key, new_item);
            return Ok(InsertResult::Success);
        }
    }
    Err(())
}

pub(crate) fn set_object(int: &mut Interpreter, mut path: Vec<String>, obj: JSItem) -> Result<InsertResult, ()> {
    let out = insert_o_r(&mut int.scopes, path, obj.clone());
    match out {
        Ok(result) => {
            match result {
                InsertResult::Ref { item } => {
                    return set_object(int, item, obj);
                }
                InsertResult::Success => {
                    return Ok(InsertResult::Success)
                }
            }
        }
        Err(_) => Err(())
    }
}