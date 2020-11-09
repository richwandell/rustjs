use crate::parser::symbols::{JSItem, Expression, Operator, Statement};
use crate::compiler::op_codes::Op;


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
                self.visit_ex(*property);
            }
            Expression::String {value} => {
                self.bc_ins.push(Op::LoadStrConst {value});
            }
            _ => {}
        }
    }

    fn visit_st(&mut self, st: Statement) {
        match st {
            Statement::AssignmentExpression { operator:_, left, right } => {
                self.visit(right);
                self.visit(left);

                let op = self.bc_ins.pop().unwrap();
                match op {
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
            _ => {}
        }
    }

    pub(crate) fn compile(&mut self, ast: JSItem) {
        self.visit(ast)
    }
}