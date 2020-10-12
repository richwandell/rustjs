use crate::parser::symbols::{JSItem, StdFun, Expression};
use crate::lexer::js_token::Tok;
use crate::vm::interpreter::Interpreter;
use crate::vm::std::inherit::inherit;
use crate::vm::scope::insert::set_object;
use crate::vm::helpers::find_object_from_reference;

pub(crate) fn array_constructor(mut arguments: Vec<JSItem>) -> Result<JSItem, ()> {
    let _this_arg = arguments.remove(0);
    let array_arg = arguments.remove(0);

    match array_arg {
        JSItem::Object { mutable:_, properties } => {
            let length = properties.get("length");
            match length {
                Some(length) => {
                    match length {
                        JSItem::Ex {expression} => {
                            match **expression {
                                Expression::Number {value} => {
                                    let mut items = vec![];
                                    let len = value.clone() as i64;
                                    for _ in 0..len {
                                        items.push(JSItem::Undefined);
                                    }

                                    Ok(JSItem::Array {
                                        items,
                                        properties: hashmap!{
                                            "length".to_string() => JSItem::Number {value: len as f64}
                                        }
                                    })
                                }
                                _ => {
                                    Err(())
                                }
                            }
                        }
                        _ => {
                            Err(())
                        }
                    }
                }
                _ => {
                    Err(())
                }
            }
        }
        JSItem::Array {items, properties } => {
            Ok(JSItem::Array {items, properties })
        }
        _ => {
            Err(())
        }
    }
}


pub(crate) fn std_array_push(interpreter: &mut Interpreter, this_path: Vec<String>, mut args: (Vec<Tok>, Vec<JSItem>)) -> Result<(), ()> {
    if let Ok(object) = find_object_from_reference(interpreter, this_path.clone()) {
        match object {
            JSItem::Variable { mutable, value } => {
                match value {
                    Expression::ArrayExpression { mut items, properties } => {
                        let mut tmp = vec![];
                        while !args.1.is_empty() {
                            tmp.push(args.1.pop().unwrap());
                        }
                        while !tmp.is_empty() {
                            items.push(tmp.pop().unwrap());
                        }
                        if let Ok(..) = set_object(interpreter, this_path, JSItem::Variable {
                            mutable,
                            value: Expression::ArrayExpression { items, properties }
                        }) {
                            return Ok(());
                        }
                    }
                    _ => return Err(())
                }
            }
            _ => {
                return Err(())
            }
        }
    }
    return Err(())
}

pub(crate) fn create_array(mut int: Interpreter) -> Interpreter {
    let or_path = vec!["Array".to_string()];
    let func = inherit(&int, JSItem::ObjectReference {path: vec!["Function".to_string()]},
                  JSItem::ObjectReference {path: or_path.clone()});

    if let JSItem::Object { mutable: _, mut properties } = func {
        if let JSItem::Object { mutable: _, properties: mut prototype } = properties.remove("prototype").unwrap() {
            prototype.insert("push".to_string(), JSItem::Std {
                params: vec![],
                func: StdFun::ArrayPush
            });

            prototype.insert("map".to_string(), JSItem::Std {
                params: vec![
                    Tok::Name { name: "callback".to_string() },
                    Tok::Name { name: "thisArg".to_string() }
                ],
                func: StdFun::ArrayMap
            });

            properties.insert("prototype".to_string(), JSItem::Object {
                mutable: false,
                properties: prototype
            });

            properties.insert("length".to_string(), JSItem::Number {value: 0.});

            properties.insert("constructor".to_string(), JSItem::Std {
                params: vec![Tok::Name{name: "items".to_string()}],
                func: StdFun::ArrayConstructor
            });

            if let Ok(..) = set_object(&mut int, or_path.clone(), JSItem::Object {
                mutable: false,
                properties
            }) {
                return int;
            }
        }
    }
    return int;
}