use std::collections::HashMap;
use crate::parser::symbols::{JSItem, StdFun};
use crate::lexer::js_token::Tok;
use crate::vm::std::inherit::inherit;
use crate::vm::interpreter::Interpreter;
use crate::vm::std::array::array_constructor;

pub(crate) fn std_fun_apply(_interpreter: &mut Interpreter, object: JSItem, args: (Vec<Tok>, Vec<JSItem>)) -> Result<JSItem, ()> {
    match object {
        JSItem::Object { mutable:_, properties } => {
            let constructor = properties.get("constructor").unwrap_or(&JSItem::Null);
            match constructor {
                JSItem::Std { params:_, func } => {
                    match func {
                        StdFun::ArrayConstructor => {
                            return array_constructor(args.1);
                        }
                        _ => {
                            return Err(());
                        }
                    }
                }
                JSItem::Function { mutable: _, params: _, properties: _, body:_ } => {
                    return Err(());
                }
                _ => {
                    return Err(());
                }
            }
        }
        _ => {
            return Err(());
        }
    }
}

pub(crate) fn create_function(mut scope: HashMap<String, JSItem>) -> HashMap<String, JSItem> {
    scope = inherit(scope, "Object".to_string(), "Function".to_string());

    let mut func = scope.remove("Function").unwrap();
    if let JSItem::Object { mutable: _, mut properties } = func {
        if let JSItem::Object { mutable: _, properties: mut prototype } = properties.remove("prototype").unwrap() {
            prototype.insert("apply".to_string(), JSItem::Std {
                params: vec![
                    Tok::Name { name: "thisArg".to_string() },
                    Tok::Name { name: "argsArray".to_string() }
                ],
                func: StdFun::FunctionApply
            });

            properties.insert("prototype".to_string(), JSItem::Object {
                mutable: false,
                properties: prototype
            });

            scope.insert("Function".to_string(), JSItem::Object {
                mutable: false,
                properties
            });
        }
    }

    return scope;
}