use crate::parser::symbols::{JSItem, Operator, Statement, StdFun};
use crate::parser::symbols::Expression;
use crate::vm::js_output::{JSOutput};
use crate::vm::bin_op::{bin_add, bin_mul, bin_sub, bin_div};
use std::collections::HashMap;
use crate::lexer::js_token::Tok;
use std::borrow::Borrow;

pub(crate) struct Interpreter {
    objects: HashMap<String, JSItem>
}

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

fn std_log(params: Vec<JSItem>) {
    for p in params {
        println!("{}", p);
    }
}

fn create_console() -> JSItem {
    let mut p = HashMap::new();
    let log = JSItem::Std {
        params: vec![Tok::Name {name: "item".to_string()}],
        func: StdFun::Log
    };
    p.insert("log".to_string(), log);
    JSItem::Object {
        mutable: false,
        properties: p
    }
}

fn create_std_objects() -> HashMap<String, JSItem> {
    let mut f = HashMap::new();
    f.insert("console".to_string(), create_console());
    return f;
}

fn find_func(objects: &HashMap<String, JSItem>, object: Box<Expression>, property: Box<Expression>) -> Option<&JSItem> {
    match *object {
        Expression::Identifier { name } => {
            let object = objects.get(&name);

            match object {
                Some(obj) => {
                    match obj {
                        JSItem::Object { mutable, properties } => {
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

impl Interpreter {

    fn call_identifier(&mut self, name: String, arguments: Vec<Tok>) -> Result<JSOutput, ()>{
        let func = self.objects.get(&name);
        match func {
            Some(f) => {
                match f {
                    JSItem::Function { mutable, params, properties, body } => {
                        let args = self.make_params(arguments);
                        let mut out = JSOutput::Null;
                        for item in body.clone() {
                            out = self.interpret(item);
                        }
                        return Ok(out);
                    }
                    _ => {
                        Err(())
                    }
                }
            }
            None => Err(())
        }
    }

    fn make_params(&self, arguments: Vec<Tok>) -> Vec<JSItem> {
        let mut params = vec![];
        for item in arguments {
            match item {
                Tok::Name {name} => {
                    let value = self.objects.get(&name).unwrap().clone();
                    params.push(value)
                }
                Tok::String {value} => {
                    params.push(JSItem::String {value});
                }
                Tok::Float {value} => {
                    params.push(JSItem::Number {value})
                }
                _ => {}
            }
        }
        return params;
    }

    fn call_member_ex(&mut self, object: Box<Expression>, property: Box<Expression>, arguments: Vec<Tok>) -> Result<JSOutput, ()>{
        let func = find_func(&self.objects, object, property);
        match func {
            Some(f) => {
                match f {
                    JSItem::Function { mutable, params, properties, body } => {
                        let mut out = JSOutput::Null;
                        for item in body.clone() {
                            out = self.interpret(item);
                        }
                        return Ok(out);
                    }
                    JSItem::Std { params, func } => {
                        match func {
                            StdFun::Log => {
                                let args = self.make_params(arguments);
                                std_log(args);
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

    pub(crate) fn new() -> Interpreter {
        Interpreter {
            objects: create_std_objects()
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
                        self.objects.insert(name.clone(), JSItem::Function {
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
                self.objects.insert(name, exp);
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