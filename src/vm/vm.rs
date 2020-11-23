use std::collections::HashMap;
use crate::parser::symbols::{JSItem, StdFun};
use crate::compiler::op_codes::Op;
use crate::vm::std::create_std_objects;
use crate::vm::scope::insert::{set_object, load_object, load_prop};
use crate::lexer::js_token::Tok;
use crate::vm::std::console::std_log;

pub(crate) struct Vm {
    ip: usize, // instruction pointer
    pub(crate) stack: Vec<JSItem>, // current instruction stack
    pub(crate) objects: HashMap<String, JSItem>,
    pub(crate) scopes: Vec<HashMap<String, String>>, // objects container
    #[cfg(test)]
    pub(crate) captured_output: Vec<Vec<JSItem>>
}

impl Vm {

    pub(crate) fn new() -> Vm {
        let mut vm = Vm {
            ip: 0,
            stack: vec![],
            objects: HashMap::new(),
            scopes: vec![HashMap::new()],
            #[cfg(test)]
            captured_output: vec![]
        };
        create_std_objects(vm)
    }

    pub(crate) fn run(&mut self, ops: Vec<Op>) -> JSItem {
        loop {
            if self.ip >= ops.len() {
                break;
            }
            let op = ops.get(self.ip).unwrap();
            match op {
                Op::DeclareFunc { start, end, mutable:_, params, name } => self.declare_func(start.clone(), end.clone(), params.clone(), name.clone()),
                Op::Return => self.return_to(),
                Op::Add => self.add(),
                Op::Sub => self.sub(),
                Op::Div => self.div(),
                Op::Mul => self.mul(),
                Op::Less => self.less(),
                Op::LoadNumConst { value } => self.load_num_const(value.clone()),
                Op::LoadStrConst { value } => self.load_str_const(value.clone()),
                Op::Store { name } => self.store(name.clone()),
                Op::Load { name } => self.load(name.clone()),
                Op::LoadMember => {}
                Op::Call { args } => self.call(args.clone()),
                Op::PopTop => {}
                Op::SetupLoop => self.setup_loop(),
                Op::PopJumpIfFalse { to } => self.pop_jump_if_false(to.clone()),
                Op::JumpAbsolute { to } => self.ip = *to,
                Op::PopBlock => self.pop_scope(),
                Op::InplaceAdd => self.in_place_add(),
                Op::LoadProp { name } => self.load_prop(name.clone()),
                Op::CreateObj => {}
            }
        }
        return self.stack.pop().unwrap_or(JSItem::Undefined);
    }

    fn return_to(&mut self) {
        match self.stack.pop().unwrap() {
            JSItem::ReturnJump { to } => {
                self.ip = to;
            }
            _ => {}
        }
    }

    #[allow(unused_must_use)]
    fn declare_func(&mut self, start: usize, end: usize, params: Vec<String>, name: String) {
        let func = JSItem::BcFunction {
            start,
            params
        };
        set_object(self, vec![name], func, true);
        self.ip = end + 1;
    }

    fn pop_scope(&mut self) {
        let scope = self.scopes.pop().unwrap();

        for key in scope.values() {
            self.objects.remove(key);
        }
        self.ip += 1;
    }

    fn in_place_add(&mut self) {
        match self.stack.pop().unwrap() {
            JSItem::Located { scope, location, object } => {
                match *object {
                    JSItem::Number { value } => {
                        self.objects.insert(location.clone(), JSItem::Located {
                            scope,
                            location,
                            object: Box::from(JSItem::Number { value: value + 1. })
                        });
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        self.ip += 1;
    }

    fn call(&mut self, args: i8) {
        let mut arguments = vec![];
        for _ in 0..args {
            arguments.push(self.get());
        }
        arguments.reverse();

        let func = self.get();
        match func {
            JSItem::Std { params, func } => {
                self.call_std(params, arguments, func);
                self.ip += 1;
            }
            JSItem::BcFunction { start, params} => {
                self.call_bcfunc(start, params, arguments)
            }
            _ => {}
        }
    }

    #[allow(unused_must_use)]
    fn call_bcfunc(&mut self, start: usize, mut params: Vec<String>, mut arguments: Vec<JSItem>) {
        self.scopes.push(HashMap::new());

        params.reverse();
        arguments.reverse();

        let mut extra = 1;
        while !arguments.is_empty() {
            let arg = arguments.pop().unwrap();
            if let Some(param) = params.pop() {
                set_object(self, vec![param], arg, true);
            } else {
                set_object(self, vec![format!("{}{}", "extra.".to_string(), extra.to_string())], arg, true);
                extra += 1;
            }
        }

        self.stack.push(JSItem::ReturnJump {to: self.ip + 1});

        self.ip = start;
    }

    fn make_params(&mut self, mut params: Vec<Tok>, mut arguments: Vec<JSItem>) -> (Vec<Tok>, Vec<JSItem>) {
        arguments.reverse();

        let mut new_params = vec![];
        while !params.is_empty() {
            let tok = params.pop().unwrap();
            match tok {
                Tok::Comma => {},
                _ => {
                    new_params.push(tok);
                }
            }
        }

        let mut names = vec![];
        let mut items = vec![];
        while !arguments.is_empty() {
            if let Some(arg) = arguments.pop() {
                names.push(new_params.pop().unwrap_or(Tok::Name {name: "extra".to_string()}));
                items.push(arg);
            }
        }

        return (names, items);
    }

    fn call_std(&mut self, params: Vec<Tok>, arguments: Vec<JSItem>, func: StdFun) {
        //create a new scope
        self.scopes.push(HashMap::new());

        match func {
            #[allow(unreachable_code)]
            StdFun::ConsoleLog => {
                let mut log = || {
                    let args = self.make_params(params, arguments);
                    #[cfg(test)]{
                        self.captured_output.push(args.1 );
                        self.scopes.pop();
                        return
                    }
                    std_log(args.1);
                    self.scopes.pop();
                    return
                };
                log();
            }
            _ => {}
            // StdFun::ObjectKeys => {
            //     self.function_scope.pop();
            //     self.remove_current_scope();
            //     return Err(());
            // }
            // StdFun::FunctionApply => {
            //     let args = self.make_params(params, arguments);
            //     let out = std_fun_apply(self, this_path, args);
            //     self.function_scope.pop();
            //     self.remove_current_scope();
            //     return out;
            // }
            // StdFun::ArrayMap => {
            //     self.function_scope.pop();
            //     self.remove_current_scope();
            //     return Err(());
            // }
            // StdFun::ArrayConstructor => {
            //     self.function_scope.pop();
            //     self.remove_current_scope();
            //     return Err(());
            // }
            // StdFun::ArrayPush => {
            //     let mut args = self.make_params(params, arguments);
            //     if let Ok(()) = std_array_push(self, this_path, args) {
            //         return Ok(JSItem::Undefined);
            //     }
            //     return Err(())
            // }
        }
    }

    #[allow(unused_must_use)]
    fn load_prop(&mut self, name: String) {
        load_prop(self,name);
        self.ip += 1;
    }

    fn pop_jump_if_false(&mut self, to: usize) {
        let value = match self.stack.pop().unwrap() {
            JSItem::Bool {value} => value,
            _ => false
        };
        if !value {
            self.ip = to;
        } else {
            self.ip += 1;
        }
    }

    fn get(&mut self) -> JSItem {
        let item = self.stack.pop().unwrap();
        return match item {
            JSItem::Located { scope, location, object } => {
                match *object {
                    JSItem::Std { params, func } => {
                        self.objects.insert(location.clone(), JSItem::Located {
                            scope,
                            location,
                            object: Box::from(JSItem::Std { params: params.clone(), func: func.clone() })
                        });
                        JSItem::Std { params, func }
                    }
                    JSItem::Number { value } => {
                        self.objects.insert(location.clone(), JSItem::Located {
                            scope,
                            location,
                            object: Box::from(JSItem::Number { value })
                        });
                        JSItem::Number { value }
                    }
                    JSItem::String { value } => {
                        self.objects.insert(location.clone(), JSItem::Located {
                            scope,
                            location,
                            object: Box::from(JSItem::String { value: value.clone() })
                        });
                        JSItem::String { value }
                    }
                    JSItem::Object { mutable, properties } => {
                        self.objects.insert(location.clone(), JSItem::Located {
                            scope,
                            location,
                            object: Box::from(JSItem::Object { mutable, properties: properties.clone() })
                        });
                        JSItem::Object { mutable, properties }
                    }
                    JSItem::BcFunction { start, params } => {
                        self.objects.insert(location.clone(), JSItem::BcFunction {
                            start: start.clone(),
                            params: params.clone()
                        });
                        JSItem::BcFunction {
                            start,
                            params
                        }
                    }
                    _ => {
                        JSItem::Null
                    }
                }
            }
            _ => {
                item
            }
        }
    }

    fn less(&mut self) {
        let v2 = match self.get() {
            JSItem::Number {value} => value,
            _ => 0.
        };
        let v1 = match self.get() {
            JSItem::Number {value} => value,
            _ => 0.
        };
        self.stack.push(JSItem::Bool {value: v1 < v2});
        self.ip += 1;
    }

    #[allow(unused_must_use)]
    fn load(&mut self, name: String) {
        load_object(self, vec![name]);
        self.ip += 1;
    }

    #[allow(unused_must_use)]
    fn store(&mut self, name: String) {
        let item = self.get();
        set_object(self, vec![name], item, true);
        self.ip += 1;
    }

    fn setup_loop(&mut self) {
        self.scopes.push(HashMap::new());
        self.ip += 1;
    }

    fn mul(&mut self) {
        let v2 = match self.get() {
            JSItem::Number {value} => value,
            _ => 0.
        };
        let v1 = match self.get() {
            JSItem::Number {value} => value,
            _ => 0.
        };
        self.stack.push(JSItem::Number {value: v1 * v2});
        self.ip += 1;
    }

    fn div(&mut self) {
        let v2 = match self.get() {
            JSItem::Number {value} => value,
            _ => 0.
        };
        let v1 = match self.get() {
            JSItem::Number {value} => value,
            _ => 0.
        };
        self.stack.push(JSItem::Number {value: v1 / v2});
        self.ip += 1;
    }

    fn sub(&mut self) {
        let v2 = match self.get() {
            JSItem::Number {value} => value,
            _ => 0.
        };
        let v1 = match self.get() {
            JSItem::Number {value} => value,
            _ => 0.
        };
        self.stack.push(JSItem::Number {value: v1 - v2});
        self.ip += 1;
    }

    fn add(&mut self) {
        let v2 = match self.get() {
            JSItem::Number {value} => value,
            _ => 0.
        };
        let v1 = match self.get() {
            JSItem::Number {value} => value,
            _ => 0.
        };
        self.stack.push(JSItem::Number {value: v1 + v2});
        self.ip += 1;
    }

    fn load_str_const(&mut self, value: String) {
        self.stack.push(JSItem::String {value});
        self.ip += 1;
    }

    fn load_num_const(&mut self, value: f64) {
        self.stack.push(JSItem::Number { value });
        self.ip += 1;
    }
}