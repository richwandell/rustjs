use crate::parser::symbols::{JSItem, Operator, Statement, StdFun};
use crate::parser::symbols::Expression;
use crate::vm::js_output::{JSOutput};
use crate::vm::bin_op::{bin_add, bin_mul, bin_sub, bin_div};
use std::collections::HashMap;
use crate::lexer::js_token::Tok;
use crate::vm::std::{std_log, create_std_objects};

fn o_to_v(js_out: JSOutput, mutable: bool) -> JSItem {
    return match js_out {
        JSOutput::String { value } => {
            JSItem::Variable {
                mutable,
                value: Expression::String { value },
            }
        }
        JSOutput::Number { value } => {
            JSItem::Variable {
                mutable,
                value: Expression::Number { value },
            }
        }
        JSOutput::Null => {
            JSItem::Variable {
                mutable,
                value: Expression::Null,
            }
        }
    };
}

fn find_func(objects: &HashMap<String, JSItem>, object: Box<Expression>, property: Box<Expression>) -> Option<&JSItem> {
    match *object {
        Expression::Identifier { name } => {
            let object = objects.get(&name);

            match object {
                Some(obj) => {
                    match obj {
                        JSItem::Object { mutable: _, properties } => {
                            match *property {
                                Expression::Identifier { name } => {
                                    let p = properties.get(&name);
                                    return p;
                                }
                                Expression::MemberExpression { object, property } => {
                                    return find_func(properties, object, property);
                                }
                                _ => {
                                    None
                                }
                            }
                        }
                        _ => {
                            None
                        }
                    }
                },
                None => None
            }
        }
        _ => None
    }
}

pub(crate) struct Interpreter {
    objects: HashMap<String, JSItem>,
    call_stack: Vec<HashMap<String, JSItem>>,
    current_call_stack: usize
}

impl Interpreter {

    pub(crate) fn new() -> Interpreter {
        Interpreter {
            objects: create_std_objects(),
            call_stack: vec![create_std_objects()],
            current_call_stack: 0
        }
    }

    fn find_object(&mut self, name: &String) -> Result<(JSItem, usize), ()> {
        for i in (0..=self.current_call_stack).rev() {
            let objects = self.call_stack.get_mut(i).unwrap();
            let object = objects.remove(name);
            if let Some(object) = object {
                return Ok((object, i));
            }
        }
        Err(())
    }

    fn add_params_to_call_stack(&mut self, mut names: Vec<Tok>, mut items: Vec<JSItem>) {
        while !items.is_empty() {
            let item = items.pop().unwrap();
            if let Tok::Name {name} = names.pop().unwrap() {
                self.call_stack.get_mut(self.current_call_stack)
                    .unwrap()
                    .insert(name, item);
            }
        }
    }

    fn call_identifier(&mut self, name: String, arguments: Vec<Tok>) -> Result<JSOutput, ()>{
        let func = self.find_object(&name);
        match func {
            Ok(f) => {
                match f.0 {
                    JSItem::Function { mutable, params, properties, body } => {
                        let body_clone = body.clone();
                        let params_clone = params.clone();
                        //first add the function back where it belongs in the call stack
                        self.call_stack.get_mut(f.1)
                            .unwrap()
                            .insert(name, JSItem::Function {
                                mutable, params, properties, body
                            });
                        //create a new stack frame for local vars
                        let new_frame = HashMap::new();
                        self.call_stack.push(new_frame);
                        self.current_call_stack += 1;
                        let args = self.make_params(params_clone, arguments);
                        self.add_params_to_call_stack(args.0, args.1);

                        let mut out = JSOutput::Null;
                        for item in body_clone {
                            out = self.interpret(item);
                        }

                        return Ok(out);
                    }
                    _ => {
                        Err(())
                    }
                }
            }
            Err(_) => Err(())
        }
    }

    fn make_params(&mut self, mut params: Vec<Tok>, mut arguments: Vec<Tok>) -> (Vec<Tok>, Vec<JSItem>) {
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
                match arg {
                    Tok::Comma => {
                        continue;
                    }
                    Tok::Name {name} => {
                        if let Ok(obj) = self.find_object(&name) {
                            items.push(obj.0.clone());
                            self.call_stack.get_mut(obj.1)
                                .unwrap()
                                .insert(name, obj.0);
                            names.push(new_params.pop().unwrap_or(Tok::Name {name: "extra".to_string()}));
                        }
                    }
                    Tok::String {value} => {
                        items.push(JSItem::String {value});
                        names.push(new_params.pop().unwrap_or(Tok::Name {name: "extra".to_string()}));
                    }
                    _ => {
                        continue;
                    }
                }
            }
        }

        return (names, items);
    }

    fn call_member_ex(&mut self, object: Box<Expression>, property: Box<Expression>, arguments: Vec<Tok>) -> Result<JSOutput, ()>{
        let func = find_func(&self.objects, object, property);
        match func {
            Some(f) => {
                match f {
                    JSItem::Function { mutable:_, params:_, properties:_, body } => {
                        let mut out = JSOutput::Null;
                        for item in body.clone() {
                            out = self.interpret(item);
                        }
                        return Ok(out);
                    }
                    JSItem::Std { params:_, func } => {
                        match func {
                            StdFun::ConsoleLog => {
                                let args = self.make_params(arguments.clone(), arguments.clone());
                                std_log(args.1);
                                return Ok(JSOutput::Null)
                            }
                        }
                    }
                    _ => {
                        Err(())
                    }
                }
            }
            None => Err(())
        }
    }

    fn visit_binop(&mut self, a: Box<Expression>, op: Operator, b: Box<Expression>) -> JSOutput {
        match op {
            Operator::Add => {
                bin_add(self.visit_ex(a), self.visit_ex(b)).unwrap()
            }
            Operator::Sub => {
                bin_sub(self.visit_ex(a), self.visit_ex(b)).unwrap()
            }
            Operator::Mult => {
                bin_mul(self.visit_ex(a), self.visit_ex(b)).unwrap()
            }
            Operator::Div => {
                bin_div(self.visit_ex(a), self.visit_ex(b)).unwrap()
            }
            _ => {
                JSOutput::Null
            }
        }
    }

    fn visit_call_ex(&mut self, callee: Box<Expression>, arguments: Vec<Tok>) -> JSOutput {
        match *callee {
            Expression::MemberExpression { object, property } => {
                self.call_member_ex(object, property, arguments).unwrap()
            }
            Expression::Identifier { name } => {
                self.call_identifier(name, arguments).unwrap()
            }
            _ => {
                JSOutput::Null
            }
        }
    }

    fn visit_ex(&mut self, ex: Box<Expression>) -> JSOutput {
        match *ex {
            Expression::Literal { value } => {
                JSOutput::String { value }
            }
            Expression::Number { value } => {
                JSOutput::Number { value }
            }
            Expression::Binop { a, op, b } => {
                self.visit_binop(a, op, b)
            }
            Expression::SubExpression { expression } => {
                self.visit_ex(expression)
            }
            Expression::CallExpression { callee, arguments } => {
                self.visit_call_ex(callee, arguments)
            }
            _ => {
                JSOutput::Null
            }
        }
    }

    fn visit_st(&mut self, st: Box<Statement>) -> JSOutput {
        match *st {
            Statement::AssignArrowFunction { mutable, function } => {
                match *function {
                    Statement::FunctionDef { name, params, body } => {
                        let mut properties = HashMap::new();
                        properties.insert("prototype".to_string(), JSItem::Ex {
                            expression: Box::new(Expression::String { value: name.clone() })
                        });
                        properties.insert("name".to_string(), JSItem::Ex {
                            expression: Box::new(Expression::Literal { value: name.clone() })
                        });
                        self.call_stack.get_mut(self.current_call_stack)
                            .unwrap()
                            .insert(name.clone(), JSItem::Function {
                            mutable,
                            properties,
                            params,
                            body,
                        });
                        JSOutput::Null
                    }
                    _ => {
                        JSOutput::Null
                    }
                }
            }
            Statement::AssignExpression { mutable, name, value } => {
                let eval = self.visit_ex(value);
                let exp = o_to_v(eval, mutable);
                self.call_stack.get_mut(self.current_call_stack)
                    .unwrap()
                    .insert(name, exp);
                JSOutput::Null
            }
            _ => {
                JSOutput::Null
            }
        }
    }

    fn visit(&mut self, tree: JSItem) -> JSOutput {
        match tree {
            JSItem::Ex { expression } => {
                self.visit_ex(expression)
            }
            JSItem::St { statement } => {
                self.visit_st(statement)
            }
            _ => {
                JSOutput::Null
            }
        }
    }

    pub(crate) fn interpret(&mut self, js_item: JSItem) -> JSOutput {
        self.visit(js_item)
    }
}