use crate::parser::symbols::{JSItem, StdFun};
use crate::lexer::js_token::Tok;
use crate::vm::std::inherit::inherit;
use crate::vm::interpreter::Interpreter;
use crate::vm::std::array::array_constructor;
use crate::vm::scope::insert::set_object;
use crate::vm::helpers::find_object_from_reference;

pub(crate) fn std_fun_apply(interpreter: &mut Interpreter, this_path: Vec<String>, args: (Vec<Tok>, Vec<JSItem>)) -> Result<JSItem, ()> {
    if let Ok(object) = find_object_from_reference(interpreter, this_path) {
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
    return Err(());
}

pub(crate) fn create_function(mut int: Interpreter) -> Interpreter {
    let or_path = vec!["Function".to_string()];
    let mut func = inherit(&int, JSItem::ObjectReference {path: vec!["Object".to_string()]},
                  JSItem::ObjectReference {path: or_path.clone()});

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