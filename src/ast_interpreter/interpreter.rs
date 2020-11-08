use crate::parser::symbols::{JSItem, Operator, Statement, StdFun, AssignOp};
use crate::parser::symbols::Expression;
use crate::ast_interpreter::bin_op::{bin_add, bin_mul, bin_sub, bin_div, bin_less};
use std::collections::HashMap;
use crate::lexer::js_token::Tok;
use crate::ast_interpreter::std::{create_std_objects};
use crate::ast_interpreter::std::console::std_log;
use crate::ast_interpreter::std::function::std_fun_apply;
use crate::ast_interpreter::helpers::{o_to_v, find_object_from_reference, find_reference_from_member_expression};
use crate::ast_interpreter::scope::insert::{set_object};
use crate::ast_interpreter::std::array::std_array_push;
use crate::ast_interpreter::std::inherit::inherit;

pub(crate) struct Interpreter {
    pub(crate) scopes: Vec<HashMap<String, JSItem>>,
    pub(crate) scope: usize,
    pub(crate) function_scope: Vec<usize>,
    #[cfg(test)]
    pub(crate) captured_output: Vec<Vec<JSItem>>
}


impl Interpreter {

    pub(crate) fn new() -> Interpreter {
        let mut int = Interpreter {
            scopes: vec![HashMap::new()],
            scope: 0,
            function_scope: vec![],
            #[cfg(test)]
            captured_output: vec![]
        };
        create_std_objects(int)
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

                        let mut out = JSItem::Undefined;
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

    fn call_function(&mut self, params: Vec<Tok>, arguments: Vec<JSItem>, body: Vec<JSItem>) -> Result<JSItem, ()> {
        //create a new scope
        self.create_new_scope();
        self.function_scope.push(self.scope.clone());
        let args = self.make_params(params, arguments);
        self.add_params_to_scope(args.0, args.1);

        let mut out = JSItem::Undefined;
        for item in body {
            out = self.interpret(item);
        }
        self.function_scope.pop();
        self.remove_current_scope();
        return Ok(out);
    }

    fn call_std(&mut self, this_path: Vec<String>, func: StdFun, params: Vec<Tok>, arguments: Vec<JSItem>) -> Result<JSItem, ()> {
        //create a new scope
        self.create_new_scope();
        self.function_scope.push(self.scope.clone());

        match func {
            #[allow(unreachable_code)]
            StdFun::ConsoleLog => {
                let args = self.make_params(params, arguments);
                #[cfg(test)]{
                    self.captured_output.push(args.1 );
                    self.function_scope.pop();
                    self.remove_current_scope();
                    return Ok(JSItem::Undefined)
                }
                std_log(args.1);
                self.function_scope.pop();
                self.remove_current_scope();
                return Ok(JSItem::Undefined)
            }
            StdFun::ObjectKeys => {
                self.function_scope.pop();
                self.remove_current_scope();
                return Err(());
            }
            StdFun::FunctionApply => {
                let args = self.make_params(params, arguments);
                let out = std_fun_apply(self, this_path, args);
                self.function_scope.pop();
                self.remove_current_scope();
                return out;
            }
            StdFun::ArrayMap => {
                self.function_scope.pop();
                self.remove_current_scope();
                return Err(());
            }
            StdFun::ArrayConstructor => {
                self.function_scope.pop();
                self.remove_current_scope();
                return Err(());
            }
            StdFun::ArrayPush => {
                let mut args = self.make_params(params, arguments);
                if let Ok(()) = std_array_push(self, this_path, args) {
                    return Ok(JSItem::Undefined);
                }
                return Err(())
            }
        }
    }

    fn call_func_ex(&mut self, ex: Expression, _this_path: Vec<String>, arguments: Vec<JSItem>) -> Result<JSItem, ()> {
        match ex {
            Expression::FuncEx { params, body } => {
                return self.call_function(params.clone(), arguments, body.clone());
            }
            _ => {
                return Err(())
            }
        }
    }

    fn call_object_reference(&mut self, this_path: Vec<String>, reference: Vec<String>, arguments: Vec<JSItem>) -> Result<JSItem, ()> {
        let mut path = reference.clone();
        let function = find_object_from_reference(self, path.clone());

        match function {
            Ok(function) => {
                match function {
                    JSItem::Ex {expression} => {
                        return self.call_func_ex(*expression, this_path, arguments);
                    }
                    JSItem::ObjectReference { path } => {
                        return self.call_object_reference( this_path, path.clone(), arguments);
                    }
                    JSItem::Function { mutable: _, params, properties: _, body } => {
                        #[allow(mutable_borrow_reservation_conflict)]
                            return self.call_function(params.clone(), arguments, body.clone());
                    }
                    JSItem::Std { params, func } => {
                        #[allow(mutable_borrow_reservation_conflict)]
                            return self.call_std(this_path, func.clone(), params.clone(), arguments);
                    }
                    _ => {
                        return Err(())
                    }
                }
            }
            Err(e) => return Err(e)
        }
    }

    fn call_member_ex(&mut self, object: Box<Expression>, property: Box<Expression>, arguments: Vec<JSItem>) -> Result<JSItem, ()>{
        let mut path = find_reference_from_member_expression(Expression::MemberExpression {object, property});
        let mut this_path = path.clone();
        this_path.pop();
        self.call_object_reference(this_path, path, arguments)
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
                JSItem::Undefined
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
                JSItem::Undefined
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
                            Expression::ArrayExpression { items, properties } => {
                                let out = JSItem::Array { items: items.clone(), properties: properties.clone() };
                                self.replace_object(obj.1, JSItem::Variable {
                                    mutable,
                                    value: Expression::ArrayExpression {items, properties}
                                }, name);
                                return out;
                            }
                            Expression::Object { mutable: om, properties } => {
                                let out = JSItem::Object {mutable: om, properties: properties.clone()};
                                self.replace_object(obj.1, JSItem::Variable {
                                    mutable,
                                    value: Expression::Object {mutable: om, properties}
                                }, name);
                                return out;
                            }
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
        JSItem::Undefined
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
        JSItem::Undefined
    }

    fn visit_member_expression(&mut self, object: Box<Expression>, property: Box<Expression>) -> JSItem {
        let object_out = self.visit_ex(object);
        match object_out {
            JSItem::Object { mutable:_, properties } => {
                if let Expression::Identifier {name} = *property {
                    if let Some(item) = properties.get(&name) {
                        return self.visit(item.clone());
                    }
                }
            }
            _ => {
                return JSItem::Undefined;
            }
        }
        JSItem::Undefined
    }

    fn visit_array_expression(&mut self, items: Vec<JSItem>, _properties: HashMap<String, JSItem>) -> JSItem {
        let mut new_items = vec![];
        for mut item in items {
            new_items.push(self.visit(item));
        }
        let array = inherit(&self, JSItem::ObjectReference {
            path: vec!["Array".to_string()]
        }, JSItem::ObjectReference {
            path: vec!["Array".to_string()]
        });

        JSItem::Array {items: new_items, properties: match array {
            JSItem::Object { mutable:_, properties } => {
                properties
            }
            _ => HashMap::new()
        }}
    }

    fn visit_ex(&mut self, ex: Box<Expression>) -> JSItem {
        match *ex {
            Expression::ArrayExpression { items, properties } => {
                self.visit_array_expression(items, properties)
            }
            Expression::MemberExpression { object, property } => {
                self.visit_member_expression(object, property)
            }
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
                JSItem::Undefined
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
        self.remove_current_scope();
        JSItem::Undefined
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

    fn assign_variable(&mut self, operator: AssignOp, left: JSItem, right: JSItem) -> Result<(), ()> {
        let mut path = vec![];
        if let JSItem::Ex {expression} = left {
            if let Expression::MemberExpression { object, property } = *expression {
                path = find_reference_from_member_expression(Expression::MemberExpression {object, property})
            } else if let Expression::String {value} = *expression {
                path = vec![value];
            } else if let Expression::Literal {value} = *expression {
                path = vec![value];
            }
        }

        let right_out = self.visit(right);
        let exp = o_to_v(right_out, operator.clone());

        match operator {
            AssignOp::Var => {
                let mut scope = 0;
                //if we are in a function, var assignments are declared at function scope
                if !self.function_scope.is_empty() {
                    scope = self.function_scope.get(self.function_scope.len() - 1).unwrap().clone();
                }
                self.scopes.get_mut(scope)
                    .unwrap()
                    .insert(path.get(0).unwrap().to_string(), exp.clone());
                return Ok(());
            }
            AssignOp::Let | AssignOp::Const => {
                if path.len() > 1 {
                    return Err(())
                }

                let scope = self.scope;
                let var_name = path.get(0).unwrap().to_string();

                //const|let cannot re declare in same scope
                if self.scopes.get(scope).unwrap().contains_key(&var_name) {
                    return Err(());
                }

                self.scopes.get_mut(scope)
                    .unwrap()
                    .insert(var_name, exp.clone());
                return Ok(());
            }
            AssignOp::None => {
                if let Ok(_result) = set_object(self, path, exp) {
                    return Ok(());
                } else {
                    return Err(());
                }
            }
        }
    }

    fn visit_st(&mut self, st: Box<Statement>) -> JSItem {
        match *st {
            Statement::Return { value } => {
                return self.visit(*value);
            }
            Statement::AssignObject { .. } => {
                JSItem::Undefined
            }
            Statement::ForStatement { init, test, update, body } => {
                return self.visit_for_statement(init, test, update, body);
            }
            Statement::AssignArrowFunction { mutable, function } => {
                match *function {
                    Statement::FunctionDef { name, params, body } => {
                        self.declare_function_in_scope(mutable, name, params, body);
                        JSItem::Undefined
                    }
                    _ => {
                        JSItem::Undefined
                    }
                }
            }
            #[allow(unused_must_use)]
            Statement::AssignmentExpression { operator, left, right } => {
                self.assign_variable(operator, left, right);
                JSItem::Undefined
            }
            Statement::FunctionDef { name, params, body } => {
                self.declare_function_in_scope(true, name, params, body);
                JSItem::Undefined
            }
            _ => {
                JSItem::Undefined
            }
        }
    }

    fn visit(&mut self, tree: JSItem) -> JSItem {
        match tree {
            JSItem::Variable { mutable:_, value } => {
                self.visit_ex(Box::from(value))
            }
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
                JSItem::Undefined
            }
        }
    }

    pub(crate) fn interpret(&mut self, js_item: JSItem) -> JSItem {
        self.visit(js_item)
    }
}