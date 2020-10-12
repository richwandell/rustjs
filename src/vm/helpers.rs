use crate::parser::symbols::{JSItem, Expression, AssignOp};
use crate::vm::interpreter::Interpreter;

pub(crate) fn o_to_v(js_out: JSItem, assign_op: AssignOp) -> JSItem {
    let mut mutable = false;
    if assign_op.eq(&AssignOp::Let) || assign_op.eq(&AssignOp::Var) {
        mutable = true;
    }
    return match js_out {
        JSItem::Array { items, properties } => {
            JSItem::Variable {
                mutable,
                value: Expression::ArrayExpression {items, properties}
            }
        }
        JSItem::Object { mutable, properties } => {
            JSItem::Variable {
                mutable,
                value: Expression::Object { mutable, properties }
            }
        }
        JSItem::String { value } => {
            JSItem::Variable {
                mutable,
                value: Expression::String { value },
            }
        }
        JSItem::Number { value } => {
            JSItem::Variable {
                mutable,
                value: Expression::Number { value },
            }
        }
        JSItem::Null => {
            JSItem::Variable {
                mutable,
                value: Expression::Null,
            }
        }
        JSItem::Bool { value } => {
            if value {
                JSItem::Variable {
                    mutable,
                    value: Expression::True,
                }
            } else {
                JSItem::Variable {
                    mutable,
                    value: Expression::False,
                }
            }
        }
        _ => {
            JSItem::Null
        }
    };
}

pub(crate) fn find_object_scope<'a>(interpreter: &'a Interpreter, name: &String) -> Result<(usize, &'a Interpreter), ()> {
    for i in (0..=interpreter.scope).rev() {
        let objects = interpreter.scopes.get(i).unwrap();
        let object = objects.get(name);
        #[allow(unused_variables)]
        if let Some(obj) = object {
            return Ok((i, interpreter));
        }
    }
    Err(())
}

pub(crate) fn find_o_r(interpreter: &Interpreter, mut scope_num: usize, mut path: Vec<String>) -> Result<JSItem, ()> {
    path.reverse();
    let mut key = path.pop().unwrap();
    let mut hashmap = interpreter.scopes.get(scope_num).unwrap();
    let mut current = hashmap.get(&key).unwrap();

    while !path.is_empty() {
        key = path.pop().unwrap();
        match current {
            JSItem::Variable { mutable:_, value } => {
                match value {
                    Expression::Object { mutable:_, properties } => {
                        let p_item = properties.get(&key);
                        if let Some(item) = p_item {
                            current = item;
                        } else {
                            return Err(())
                        }
                    }
                    Expression::ArrayExpression { items:_, properties } => {
                        let item = properties.get(&key);
                        match item {
                            Some(i) => {
                                current = i;
                            }
                            None => {
                                let prototype = properties.get("prototype");
                                match prototype {
                                    None => {}
                                    Some(item) => {
                                        match item {
                                            JSItem::Object { mutable: _, properties } => {
                                                let p_item = properties.get(&key);
                                                if let Some(item) = p_item {
                                                    current = item;
                                                }
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
                    _ => {
                        return Err(())
                    }
                }
            }
            JSItem::ObjectReference { path: _ } => {
                // path = new_path.clone();
                // let scope_return = find_object_scope(&interpreter, new_path.get(0).unwrap()).unwrap();
                // scope_num = scope_return.0;
                // hashmap = interpreter.scopes.get(scope_num).unwrap();
            }
            JSItem::Object { mutable: _, properties } => {
                let item = properties.get(&key);
                match item {
                    Some(i) => {
                        current = i;
                    }
                    None => {
                        let prototype = properties.get("prototype");
                        match prototype {
                            None => {}
                            Some(item) => {
                                match item {
                                    JSItem::Object { mutable: _, properties } => {
                                        let p_item = properties.get(&key);
                                        if let Some(item) = p_item {
                                            current = item;
                                        }
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
            _ => {
                return Err(());
            }
        }
    }
    Ok(current.clone())
}

pub(crate) fn find_object_from_reference(int: &Interpreter, mut path: Vec<String>) -> Result<JSItem, ()> {
    let name = path.get(0).unwrap();
    let scope_num_option = find_object_scope(int, name);
    match scope_num_option {
        Ok(scope_return) => {
            let scope_num = scope_return.0;
            find_o_r(int, scope_num, path)
        }
        Err(e) => Err(e)
    }
}

pub(crate) fn find_reference_from_member_expression(exp: Expression) -> Vec<String> {
    let mut full = vec![];
    if let Expression::MemberExpression { object, property } = exp {
        match *object {
            Expression::Identifier {name} => {
                full.push(name)
            }
            Expression::MemberExpression {object, property} => {
                let mut n = find_reference_from_member_expression(Expression::MemberExpression {object, property});
                full.append(&mut n);
            }
            _ => {}
        }
        match *property {
            Expression::Identifier {name} => {
                full.push(name)
            }
            _ => {}
        }
    }
    return full;
}

