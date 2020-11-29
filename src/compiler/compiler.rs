use crate::parser::symbols::{JSItem, Expression, Operator, Statement};
use crate::compiler::op_codes::Op;
use crate::lexer::js_token::Tok;
use std::collections::HashMap;


pub(crate) struct Compiler {
    pub(crate) bc_ins: Vec<Op>
}


impl Compiler {

    pub(crate) fn new() -> Compiler {
        Compiler {
            bc_ins: Vec::default()
        }
    }

    fn visit_binop(&mut self, a: Expression, op: Operator, b: Expression) {
        let top = match op {
            Operator::Add => Op::Add,
            Operator::Sub => Op::Sub,
            Operator::Mult => Op::Mul,
            Operator::Div => Op::Div,
            Operator::Less => Op::Less,
            Operator::Greater => Op::Greater,
            _ => Op::Add
        };
        self.visit_ex(a);
        self.visit_ex(b);
        self.bc_ins.push(top);
    }

    fn visit_ex(&mut self, ex: Expression) {
        match ex {
            Expression::UpdateExpression { expression } => {
                self.visit_ex(*expression);
                self.bc_ins.push(Op::InplaceAdd);
            }
            Expression::Binop { a, op, b } => {
                self.visit_binop(*a, op, *b)
            }
            Expression::Number { value } => {
                self.bc_ins.push(Op::LoadNumConst {value})
            }
            Expression::Literal { value } => {
                self.bc_ins.push(Op::LoadStrConst {value})
            }
            Expression::Identifier { name } => {
                self.bc_ins.push(Op::Load { name })
            }
            Expression::SubExpression { expression } => {
                self.visit_ex(*expression)
            }
            Expression::CallExpression { callee, arguments } => {
                self.visit_ex(*callee);
                let arg_len = arguments.len().clone();
                for item in arguments {
                    self.visit(item);
                }
                self.bc_ins.push(Op::Call { args: arg_len as i8 });
            }
            Expression::MemberExpression { object, property } => {
                self.visit_ex(*object);
                let prop = match *property {
                    Expression::Identifier { name } => name,
                    _ => "".to_string()
                };
                self.bc_ins.push(Op::LoadProp {name: prop})
            }
            Expression::String {value} => {
                self.bc_ins.push(Op::LoadStrConst {value});
            }
            _ => {}
        }
    }

    fn visit_st(&mut self, st: Statement) {
        match st {
            Statement::AssignFunction { mutable, function } => {
                let func_start = self.bc_ins.len();
                self.visit_st(*function);

                match self.bc_ins.get(func_start).unwrap() {
                    Op::DeclareFunc { start, end, mutable: _, params, name } => {
                        self.bc_ins[func_start] = Op::DeclareFunc {
                            start: start.clone(),
                            end: end.clone(),
                            mutable,
                            params: params.clone(),
                            name: name.clone()
                        }
                    }
                    _ => {}
                }
            }
            Statement::FunctionDef { name, params, body } => {
                let mut prams = vec![];
                for p in params {
                    match p {
                        Tok::Name { name } => prams.push(name),
                        Tok::String { value } => prams.push(value),
                        _ => {}
                    }
                }

                let func_start = self.bc_ins.len();

                self.bc_ins.push(Op::DeclareFunc {
                    mutable: true,
                    name: name.clone(),
                    start: 0,
                    end: 0,
                    params: vec![]
                });

                for item in body {
                    self.visit(item);
                }

                self.bc_ins.push(Op::PopBlock);
                self.bc_ins.push(Op::Return);

                self.bc_ins[func_start] = Op::DeclareFunc {
                    start: func_start + 1,
                    end: self.bc_ins.len() - 1,
                    mutable: true,
                    params: prams,
                    name
                }
            }
            Statement::AssignmentExpression { operator:_, left, right } => {
                self.visit(left);
                let op = self.bc_ins.pop().unwrap();
                self.visit(right);

                match op {
                    Op::LoadProp { name } => {
                        self.bc_ins.push(Op::StoreProp { name });
                        self.bc_ins.push(Op::PopTop)
                    }
                    Op::LoadStrConst { value } => {
                        self.bc_ins.push(Op::Store {name: value})
                    }
                    _ => {}
                }
            }
            Statement::ForStatement { init, test, update, body } => {
                self.bc_ins.push(Op::SetupLoop);
                self.visit(init);

                let test_start = self.bc_ins.len();
                self.visit(test);
                let pop_jump_i = self.bc_ins.len();
                self.bc_ins.push(Op::PopJumpIfFalse {to: 0});

                for item in body {
                    self.visit(item);
                }

                self.visit(update);
                self.bc_ins.push(Op::JumpAbsolute {to: test_start });

                let jump_to = self.bc_ins.len();
                self.bc_ins[pop_jump_i] = Op::PopJumpIfFalse {to: jump_to};

                self.bc_ins.push(Op::PopBlock);
            }
            Statement::If { test, consequent, alternate } => {
                self.visit(test);
                let pop_jump_i = self.bc_ins.len();
                self.bc_ins.push(Op::PopJumpIfFalse {to: 0});

                for item in consequent {
                    self.visit(item);
                }
                self.bc_ins.push(Op::JumpAbsolute { to: 0 });

                let mut jump_to = self.bc_ins.len();
                self.bc_ins[pop_jump_i] = Op::PopJumpIfFalse {to: jump_to};
                let jump_to_i = jump_to - 1;

                self.visit(alternate);
                jump_to = self.bc_ins.len();
                self.bc_ins[jump_to_i] = Op::JumpAbsolute { to: jump_to};
            }
            _ => {}
        }
    }

    fn visit(&mut self, item: JSItem) {
        match item {
            JSItem::Ex { expression } => {
                self.visit_ex(*expression)
            }
            JSItem::St { statement } => {
                self.visit_st(*statement)
            }
            JSItem::Object { mutable, properties } => {
                self.visit_object(mutable, properties)
            }
            _ => {}
        }
    }

    fn visit_object(&mut self, _mutable: bool, mut properties: HashMap<String, JSItem>) {
        self.bc_ins.push(Op::CreateObj);

        let mut keys = vec![];
        for key in properties.keys() {
            keys.push(key.clone())
        }

        for key in keys {
            let item = properties.remove(&key).unwrap();

            self.visit(item);
            self.bc_ins.push(Op::StoreProp {name: key.clone() });
        }
    }

    pub(crate) fn compile(&mut self, ast: JSItem) {
        self.visit(ast)
    }
}