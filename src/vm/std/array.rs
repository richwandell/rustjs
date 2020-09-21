use std::collections::HashMap;
use crate::parser::symbols::{JSItem, StdFun, Expression};
use crate::lexer::js_token::Tok;
use crate::vm::std::inherit::inherit;

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
                                    Ok(JSItem::Array {items, length: len as usize })
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
        JSItem::Array {items, length } => {
            Ok(JSItem::Array {items, length })
        }
        _ => {
            Err(())
        }
    }
}

pub(crate) fn create_array(mut scope: HashMap<String, JSItem>) -> HashMap<String, JSItem> {
    scope = inherit(scope, "Function".to_string(), "Array".to_string());

    let mut func = scope.remove("Array").unwrap();
    if let JSItem::Object { mutable: _, mut properties } = func {
        if let JSItem::Object { mutable: _, properties: mut prototype } = properties.remove("prototype").unwrap() {

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

            scope.insert("Array".to_string(), JSItem::Object {
                mutable: false,
                properties
            });
        }
    }
    return scope;
}