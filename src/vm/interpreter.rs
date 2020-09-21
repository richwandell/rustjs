use crate::parser::symbols::{JSItem, Operator, Statement, StdFun, AssignOp};
use crate::parser::symbols::Expression;
use crate::vm::bin_op::{bin_add, bin_mul, bin_sub, bin_div, bin_less};
use std::collections::HashMap;
use crate::lexer::js_token::Tok;
use crate::vm::std::{create_std_objects};
use crate::vm::std::console::std_log;
use crate::vm::std::function::std_fun_apply;

fn o_to_v(js_out: JSItem, assign_op: AssignOp) -> JSItem {
    let mut mutable = false;
    if assign_op.eq(&AssignOp::Let) || assign_op.eq(&AssignOp::Var) {
        mutable = true;
    }
    return match js_out {
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
                    value: Expression::True
                }
            } else {
                JSItem::Variable {
                    mutable,
                    value: Expression::False
                }
            }
        }
        _ => {
            JSItem::Null
        }
    };
}

fn find_func(objects: &HashMap<String, JSItem>, object: Box<Expression>, property: Box<Expression>) -> Option<(&JSItem, &JSItem)> {
    match *object {
        Expression::Identifier { name: object_name } => {
            let object = objects.get(&object_name);

            match object {
                Some(obj) => {
                    match obj {
                        JSItem::ObjectReference { path:_ } => {
                            return Some((obj, obj));
                        }
                        JSItem::Object { mutable:_, properties } => {
                            match *property {
                                Expression::Identifier { name: property_name } => {
                                    let p = properties.get(&property_name);
                                    match p {
                                        Some(p) => {
                                            return Some((obj, p));
                                        }
                                        None => {
                                            let prototype = properties.get("prototype");
                                            match prototype {
                                                Some(proto) => {
                                                    match proto {
                                                        JSItem::Object { mutable:_, properties } => {
                                                            let constructor = obj;
                                                            let next_proto = find_func(
                                                                properties,
                                                                Box::from(Expression::Identifier {name: property_name}),
                                                                Box::from(Expression::None)
                                                            );
                                                            match next_proto {
                                                                Some(proto) => {
                                                                    return Some((constructor, proto.1))
                                                                }
                                                                None => {
                                                                    return None
                                                                }
                                                            }
                                                        }
                                                        _ => {
                                                            return None
                                                        }
                                                    }
                                                }
                                                None => {
                                                    return None
                                                }
                                            }
                                        }
                                    }
                                }
                                Expression::MemberExpression { object, property } => {
                                    return find_func(properties, object, property);
                                }
                                Expression::None => {
                                    return Some((obj, obj));
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
    pub(crate) scopes: Vec<HashMap<String, JSItem>>,
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

    fn find_object_scope(&self, name: &String) -> Result<usize, ()> {
        for i in (0..=self.scope).rev() {
            let objects = self.scopes.get(i).unwrap();
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

    fn call_identifier(&mut self, name: String, arguments: Vec<JSItem>) -> Result<JSItem, ()>{
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

                        let mut out = JSItem::Null;
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
                let out = self.visit(arg);
                names.push(new_params.pop().unwrap_or(Tok::Name {name: "extra".to_string()}));
                items.push(out);
            }
        }

        return (names, items);
    }

    fn find_object_reference(&self, scope_num: usize, mut path: Vec<String>) -> Result<(JSItem, JSItem), ()> {
        path.reverse();
        let mut key = path.pop().unwrap();
        let mut hashmap = self.scopes.get(scope_num).unwrap();
        let mut current = hashmap.get(&key).unwrap();
        let mut last = current;
        while !path.is_empty() {
            key = path.pop().unwrap();
            match current {
                JSItem::ObjectReference { path } => {
                    let scope_num = self.find_object_scope(path.get(0).unwrap()).unwrap();
                    return self.find_object_reference(scope_num, path.clone());
                }
                JSItem::Object { mutable:_, properties } => {
                    let item  = properties.get(&key);
                    match item {
                        Some(i) => {
                            last = current;
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
                                                last = current;
                                                current = item;
                                            }
                                        }
                                        _ => {
                                            return Err(())
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
        Ok((last.clone(), current.clone()))
    }

    fn call_function(&mut self, params: Vec<Tok>, arguments: Vec<JSItem>, body: Vec<JSItem>) -> Result<JSItem, ()> {
        //create a new scope
        self.create_new_scope();
        let args = self.make_params(params, arguments);
        self.add_params_to_scope(args.0, args.1);

        let mut out = JSItem::Null;
        for item in body {
            out = self.interpret(item);
        }
        return Ok(out);
    }

    fn call_std(&mut self, object: JSItem, func: StdFun, params: Vec<Tok>, arguments: Vec<JSItem>) -> Result<JSItem, ()> {
        match func {
            #[allow(unreachable_code)]
            StdFun::ConsoleLog => {
                //create a new scope
                self.create_new_scope();
                let args = self.make_params(params, arguments);
                #[cfg(test)]{
                    self.captured_output.push(args.1 );
                    return Ok(JSItem::Null)
                }
                std_log(args.1);
                return Ok(JSItem::Null)
            }
            StdFun::ObjectKeys => {
                return Err(());
            }
            StdFun::FunctionApply => {
                //create a new scope
                self.create_new_scope();
                let args = self.make_params(params, arguments);
                return std_fun_apply(self, object, args);
            }
            StdFun::ArrayMap => {
                return Err(());
            }
            StdFun::ArrayConstructor => {
                return Err(());
            }
        }
    }

    fn call_member_ex(&mut self, object: Box<Expression>, property: Box<Expression>, arguments: Vec<JSItem>) -> Result<JSItem, ()>{
        if let Expression::Identifier {name} = *object {
            let scope_num_option = self.find_object_scope(&name);
            match scope_num_option {
                Ok(scope_num) => {
                    let scope = self.scopes.get(scope_num).unwrap();
                    let cobj_func = find_func(scope, Box::new(Expression::Identifier {name: name.clone()}), property);
                    match cobj_func {
                        Some(f) => {
                            match f.1 {
                                JSItem::ObjectReference { path } => {
                                    let cobj_func = self.find_object_reference(scope_num, path.clone());
                                    match cobj_func {
                                        Ok(f1) => {
                                            if let JSItem::Function { mutable: _, params, properties:_, body } = f1.1 {
                                                return self.call_function(params.clone(), arguments, body.clone());
                                            }
                                            if let JSItem::Std { params, func } = f1.1 {
                                                #[allow(mutable_borrow_reservation_conflict)]
                                                return self.call_std(f.0.clone(), func.clone(), params.clone(), arguments);
                                            }
                                            return Err(())
                                        }
                                        Err(_) => {
                                            return Err(())
                                        }
                                    }
                                }
                                JSItem::Function { mutable:_, params, properties:_, body } => {
                                    #[allow(mutable_borrow_reservation_conflict)]
                                    return self.call_function(params.clone(), arguments, body.clone());
                                }
                                JSItem::Std { params, func } => {
                                    #[allow(mutable_borrow_reservation_conflict)]
                                    return self.call_std(f.0.clone(), func.clone(), params.clone(), arguments);
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

    fn visit_binop(&mut self, a: Box<Expression>, op: Operator, b: Box<Expression>) -> JSItem {
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
                JSItem::Null
            }
        }
    }

    fn visit_call_ex(&mut self, callee: Box<Expression>, arguments: Vec<JSItem>) -> JSItem {
        match *callee {
            Expression::MemberExpression { object, property } => {
                self.call_member_ex(object, property, arguments).unwrap()
            }
            Expression::Identifier { name } => {
                self.call_identifier(name, arguments).unwrap()
            }
            _ => {
                JSItem::Null
            }
        }
    }

    fn visit_ident(&mut self, name: String) -> JSItem {
        let object = self.get_object(&name);
        match object {
            Ok(obj) => {
                match obj.0 {
                    JSItem::Variable { mutable, value } => {
                        match value {
                            Expression::String {value} => {
                                let out = JSItem::String {value: value.clone()};
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
                                return JSItem::Number {value: value.clone()};
                            }
                            _ => {}
                        }
                    }
                    JSItem::Number {value} => {
                        self.replace_object(obj.1, JSItem::Number {value}, name);
                        return JSItem::Number {value: value.clone()};
                    }
                    JSItem::String {value} => {
                        let out = JSItem::String {value: value.clone()};
                        self.replace_object(obj.1, JSItem::String {value}, name);
                        return out;
                    }
                    _ => {}
                }
            },
            Err(_) => {}
        }
        JSItem::Null
    }

    fn visit_ex_up(&mut self, ex: Box<Expression>) -> JSItem {
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
        JSItem::Null
    }

    fn visit_ex(&mut self, ex: Box<Expression>) -> JSItem {
        match *ex {
            Expression::UpdateExpression {expression} => {
                self.visit_ex_up(expression)
            }
            Expression::Identifier {name} => {
                self.visit_ident(name)
            }
            Expression::Literal { value } => {
                JSItem::String { value }
            }
            Expression::Number { value } => {
                JSItem::Number { value }
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
                JSItem::String {value}
            }
            _ => {
                JSItem::Null
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

    fn visit_for_statement(&mut self, init: JSItem, test: JSItem, update: JSItem, body: Vec<JSItem>) -> JSItem {
        self.create_new_scope();
        self.visit(init);
        loop {
            let cloned_test = test.clone();
            let test_out = self.visit(cloned_test);
            if let JSItem::Bool {value} = test_out {
                if !value {
                    break;
                }
            }

            for item in body.clone() {
                self.interpret(item);
            }

            self.visit(update.clone());
        }
        JSItem::Null
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

    fn visit_st(&mut self, st: Box<Statement>) -> JSItem {
        match *st {
            Statement::ForStatement { init, test, update, body } => {
                return self.visit_for_statement(init, test, update, body);
            }
            Statement::AssignArrowFunction { mutable, function } => {
                match *function {
                    Statement::FunctionDef { name, params, body } => {
                        self.declare_function_in_scope(mutable, name, params, body);
                        JSItem::Null
                    }
                    _ => {
                        JSItem::Null
                    }
                }
            }
            Statement::AssignExpression { assign_op, name, value } => {
                let eval = self.visit_ex(value);
                let exp = o_to_v(eval, assign_op);
                self.scopes.get_mut(self.scope)
                    .unwrap()
                    .insert(name, exp);
                JSItem::Null
            }
            Statement::FunctionDef { name, params, body } => {
                self.declare_function_in_scope(true, name, params, body);
                JSItem::Null
            }
            _ => {
                JSItem::Null
            }
        }
    }

    fn visit(&mut self, tree: JSItem) -> JSItem {
        match tree {
            JSItem::Ex { expression } => {
                self.visit_ex(expression)
            }
            JSItem::St { statement } => {
                self.visit_st(statement)
            }
            JSItem::Object { mutable, properties } => {
                JSItem::Object {mutable, properties}
            }
            _ => {
                JSItem::Null
            }
        }
    }

    pub(crate) fn interpret(&mut self, js_item: JSItem) -> JSItem {
        self.visit(js_item)
    }
}