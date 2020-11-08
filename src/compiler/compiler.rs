use crate::parser::symbols::{JSItem, Expression, Operator};

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Op {
    Return,
    Add,
    Sub,
    Div,
    Mul,
    PushNum {
        value: f64
    }
}


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
        match op {
            Operator::Add => {
                self.visit_ex(a);
                self.visit_ex(b);
                self.bc_ins.push(Op::Add)
            }
            Operator::Sub => {
                self.visit_ex(a);
                self.visit_ex(b);
                self.bc_ins.push(Op::Sub)
            }
            Operator::Mult => {
                self.visit_ex(a);
                self.visit_ex(b);
                self.bc_ins.push(Op::Mul)
            }
            Operator::Div => {
                self.visit_ex(a);
                self.visit_ex(b);
                self.bc_ins.push(Op::Div)
            }
            _ => {}
        }
    }

    fn visit_ex(&mut self, ex: Expression) {
        match ex {
            Expression::Binop { a, op, b } => {
                self.visit_binop(*a, op, *b)
            }
            Expression::Number { value } => {
                self.bc_ins.push(Op::PushNum {value})
            }
            Expression::SubExpression { expression } => {
                self.visit_ex(*expression)
            }
            _ => {}
        }
    }

    pub(crate) fn compile(&mut self, ast: JSItem) {
        match ast {
            JSItem::Ex { expression } => {
                self.visit_ex(*expression)
            }
            JSItem::St { statement } => {

            }
            _ => {}
        }
    }
}