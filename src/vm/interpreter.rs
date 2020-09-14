use crate::parser::symbols::{JSItem, Operator, Statement, StdFun, AssignOp};
use crate::parser::symbols::Expression;
use crate::vm::js_output::{JSOutput};
use crate::vm::bin_op::{bin_add, bin_mul, bin_sub, bin_div, bin_less};
use std::collections::HashMap;
use crate::lexer::js_token::Tok;
use crate::vm::std::{std_log, create_std_objects};

fn o_to_v(js_out: JSOutput, assign_op: AssignOp) -> JSItem {
    let mut mutable = false;
    if assign_op.eq(&AssignOp::Let) || assign_op.eq(&AssignOp::Var) {
        mutable = true;
    }
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
        JSOutput::Bool { value } => {
            if value {
                JSItem::Variable {
                    mutable,
                    value: Expression::True
                }
            } else {
                JSItem::Variable {
                    mutable,
                    value: Expression::False
                }
            }
        }
    };
}

fn o_to_i(js_out: JSOutput) -> JSItem {
    return match js_out {
        JSOutput::String { value } => {
            JSItem::String { value }
        }
        JSOutput::Number { value } => {
            JSItem::Number { value }
        }
        JSOutput::Null => {
            JSItem::Null
        }
        JSOutput::Bool { value } => {
            JSItem::Bool { value }
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
    scopes: Vec<HashMap<String, JSItem>>,
    scope: usize,
    #[cfg(test)]
    pub(crate) captured_output: Vec<Vec<JSItem>>
}

impl Interpreter {

    pub(crate) fn new() -> Interpreter {
        Interpreter {
            scopes: vec![create_std_objects()],
            scope: 0,
            #[cfg(test)]
            captured_output: vec![]
        }
    }

    fn get_object(&mut self, name: &String) -> Result<(JSItem, usize), ()> {
        for i in (0..=self.scope).rev() {
            let objects = self.scopes.get_mut(i).unwrap();
            let object = objects.remove(name);
            if let Some(object) = object {
                return Ok((object, i));
            }
        }
        Err(())
    }

    fn find_object_scope(&mut self, name: &String) -> Result<usize, ()> {
        for i in (0..=self.scope).rev() {
            let objects = self.scopes.get_mut(i).unwrap();
            let object = objects.get(name);
            #[allow(unused_variables)]
            if let Some(obj) = object {
                return Ok(i);
            }
        }
        Err(())
    }

    fn replace_object(&mut self, scope: usize, object: JSItem, name: String) {
        self.scopes.get_mut(scope)
            .unwrap()
            .insert(name, object);
    }

    fn add_params_to_scope(&mut self, mut names: Vec<Tok>, mut items: Vec<JSItem>) {
        while !items.is_empty() {
            let item = items.pop().unwrap();
            if let Tok::Name {name} = names.pop().unwrap() {
                self.scopes.get_mut(self.scope)
                    .unwrap()
                    .insert(name, item);
            }
        }
    }

    fn call_identifier(&mut self, name: String, arguments: Vec<JSItem>) -> Result<JSOutput, ()>{
        let func = self.get_object(&name);
        match func {
            Ok(f) => {
                match f.0 {
                    JSItem::Function { mutable, params, properties, body } => {
                        let body_clone = body.clone();
                        let params_clone = params.clone();
                        //first add the function back where it belongs in the call stack
                        self.scopes.get_mut(f.1)
                            .unwrap()
                            .insert(name, JSItem::Function {
                                mutable, params, properties, body
                            });
                        //create a new scope
                        self.create_new_scope();
                        let args = self.make_params(params_clone, arguments);
                        self.add_params_to_scope(args.0, args.1);

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
                let out = o_to_i(self.visit(arg));
                names.push(new_params.pop().unwrap_or(Tok::Name {name: "extra".to_string()}));
                items.push(out);
            }
        }

        return (names, items);
    }

    fn call_member_ex(&mut self, object: Box<Expression>, property: Box<Expression>, arguments: Vec<JSItem>) -> Result<JSOutput, ()>{
        if let Expression::Identifier {name} = *object {
            let scope = self.find_object_scope(&name);
            match scope {
                Ok(s) => {
                    let scope = self.scopes.get_mut(s).unwrap();
                    let func = find_func(scope, Box::new(Expression::Identifier {name: name.clone()}), property);
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
                                        #[allow(unreachable_code)]
                                        StdFun::ConsoleLog => {
                                            let args = self.make_params(vec![], arguments.clone());
                                            #[cfg(test)]{
                                                self.captured_output.push(args.1 );
                                                return Ok(JSOutput::Null)
                                            }
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
                Err(e) => return Err(e)
            }
        } else {
            Err(())
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
            Operator::Less => {
                bin_less(self.visit_ex(a), self.visit_ex(b)).unwrap()
            }
            _ => {
                JSOutput::Null
            }
        }
    }

    fn visit_call_ex(&mut self, callee: Box<Expression>, arguments: Vec<JSItem>) -> JSOutput {
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

    fn visit_ident(&mut self, name: String) -> JSOutput {
        let object = self.get_object(&name);
        match object {
            Ok(obj) => {
                match obj.0 {
                    JSItem::Variable { mutable, value } => {
                        match value {
                            Expression::String {value} => {
                                let out = JSOutput::String {value: value.clone()};
                                self.replace_object(obj.1, JSItem::Variable {
                                    mutable,
                                    value: Expression::String {value}
                                }, name);
                                return out;
                            }
                            Expression::Number {value} => {
                                self.replace_object(obj.1, JSItem::Variable {
                                    mutable,
                                    value: Expression::Number {value}
                                }, name);
                                return JSOutput::Number {value: value.clone()};
                            }
                            _ => {}
                        }
                    }
                    JSItem::Number {value} => {
                        self.replace_object(obj.1, JSItem::Number {value}, name);
                        return JSOutput::Number {value: value.clone()};
                    }
                    JSItem::String {value} => {
                        let out = JSOutput::String {value: value.clone()};
                        self.replace_object(obj.1, JSItem::String {value}, name);
                        return out;
                    }
                    _ => {}
                }
            },
            Err(_) => {}
        }
        JSOutput::Null
    }

    fn visit_ex_up(&mut self, ex: Box<Expression>) -> JSOutput {
        if let Expression::Identifier {name} = *ex {
            if let Ok(obj) = self.get_object(&name) {
                if let JSItem::Variable {mutable, value} = obj.0 {
                    match value {
                        Expression::String {value:_} => {
                            self.replace_object(obj.1, JSItem::NaN, name);
                        }
                        Expression::Number {value} => {
                            self.replace_object(obj.1, JSItem::Variable {
                                mutable,
                                value: Expression::Number {value: value + 1.}
                            }, name);
                        }
                        _ => {}
                    }
                }
            }
        }
        JSOutput::Null
    }

    fn visit_ex(&mut self, ex: Box<Expression>) -> JSOutput {
        match *ex {
            Expression::UpdateExpression {expression} => {
                self.visit_ex_up(expression)
            }
            Expression::Identifier {name} => {
                self.visit_ident(name)
            }
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
            Expression::String {value} => {
                JSOutput::String {value}
            }
            _ => {
                JSOutput::Null
            }
        }
    }

    fn create_new_scope(&mut self) {
        let scope = HashMap::new();
        self.scopes.push(scope);
        self.scope += 1;
    }

    #[allow(dead_code)]
    fn remove_current_scope(&mut self) {
        self.scopes.pop();
        self.scope -= 1;
    }

    fn visit_for_statement(&mut self, init: JSItem, test: JSItem, update: JSItem, body: Vec<JSItem>) -> JSOutput {
        self.create_new_scope();
        self.visit(init);
        loop {
            let cloned_test = test.clone();
            let test_out = self.visit(cloned_test);
            if let JSOutput::Bool {value} = test_out {
                if !value {
                    break;
                }
            }

            for item in body.clone() {
                self.interpret(item);
            }

            self.visit(update.clone());
        }
        JSOutput::Null
    }

    fn declare_function_in_scope(&mut self, mutable: bool, name: String, params: Vec<Tok>, body: Vec<JSItem>) {
        let mut properties = HashMap::new();
        properties.insert("prototype".to_string(), JSItem::Ex {
            expression: Box::new(Expression::String { value: name.clone() })
        });
        properties.insert("name".to_string(), JSItem::Ex {
            expression: Box::new(Expression::Literal { value: name.clone() })
        });
        self.scopes.get_mut(self.scope)
            .unwrap()
            .insert(name.clone(), JSItem::Function {
                mutable,
                properties,
                params,
                body,
            });
    }

    fn visit_st(&mut self, st: Box<Statement>) -> JSOutput {
        match *st {
            Statement::ForStatement { init, test, update, body } => {
                return self.visit_for_statement(init, test, update, body);
            }
            Statement::AssignArrowFunction { mutable, function } => {
                match *function {
                    Statement::FunctionDef { name, params, body } => {
                        self.declare_function_in_scope(mutable, name, params, body);
                        JSOutput::Null
                    }
                    _ => {
                        JSOutput::Null
                    }
                }
            }
            Statement::AssignExpression { assign_op, name, value } => {
                let eval = self.visit_ex(value);
                let exp = o_to_v(eval, assign_op);
                self.scopes.get_mut(self.scope)
                    .unwrap()
                    .insert(name, exp);
                JSOutput::Null
            }
            Statement::FunctionDef { name, params, body } => {
                self.declare_function_in_scope(true, name, params, body);
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