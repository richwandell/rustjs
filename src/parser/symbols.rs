use crate::lexer::js_token::Tok;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Expression {
    Object {
        mutable: bool,
        properties: HashMap<String, JSItem>
    },
    Binop {
        a: Box<Expression>,
        op: Operator,
        b: Box<Expression>,
    },
    // A numeric literal.
    Number {
        value: f64,
    },
    String {
        value: String
    },
    Identifier {
        name: String
    },
    Literal {
        value: String
    },
    Null,
    None,
    True, // The literal 'True'.
    False, // The literal 'False'.
    CallExpression {
        callee: Box<Expression>,
        arguments: Vec<JSItem>
    },
    MemberExpression {
        object: Box<Expression>,
        property: Box<Expression>
    },
    SubExpression {
       expression: Box<Expression>
    },
    UpdateExpression {
        expression: Box<Expression>
    },
    ArrayExpression {
        items: Vec<JSItem>,
        properties: HashMap<String, JSItem>
    },
    FuncEx {
        params: Vec<Tok>,
        body: Vec<JSItem>
    },
}

/// An operator for a binary operation (an operation with two operands).
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Operator {
    None, // temporary none
    Add, // +
    Sub, // -
    Mult, // *
    Div, // /
    Mod, // %
    LShift, // <<
    RShift, // >>
    BitOr, // |
    BitXor, //
    BitAnd,
    FloorDiv,
    Less, // <
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum AssignOp {
    Let,
    Const,
    Var,
    None
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Statement {
    //temporary non filled in statement
    None,

    Break,

    Continue,

    Return {
        value: Box<JSItem>
    },

    AssignmentExpression {
        operator: AssignOp,
        left: JSItem,
        right: JSItem
    },

    AssignObject {
        assign_op: AssignOp,
        name: String,
        value: JSItem
    },

    /// An expression used as a statement.
    Expression {
        expression: Box<Expression>
    },

    If {
        test: Box<Expression>,
        body: Box<Expression>
    },

    While {
        test: Box<Expression>,
        body: Box<Expression>,
    },

    FunctionDef {
        name: String,
        params: Vec<Tok>,
        body: Vec<JSItem>
    },

    AssignArrowFunction {
        mutable: bool,
        function: Box<Statement>
    },

    AssignFunction {
        mutable: bool,
        function: Box<Statement>
    },

    ForStatement {
        init: JSItem,
        test: JSItem,
        update: JSItem,
        body: Vec<JSItem>
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum StdFun {
    ConsoleLog,
    ObjectKeys,
    FunctionApply,
    ArrayMap,
    ArrayConstructor,
    ArrayPush
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum JSItem {
    ObjectReference {
        path: Vec<String>
    },
    Bool {
        value: bool
    },
    Null,
    Undefined,
    Number {
        value: f64
    },
    NaN,
    String {
        value: String
    },

    Std {
        params: Vec<Tok>,
        func: StdFun
    },

    Ex {
        expression: Box<Expression>
    },

    St {
        statement: Box<Statement>
    },

    Object {
        mutable: bool,
        properties: HashMap<String, JSItem>
    },

    Located {
        scope: usize,
        location: String,
        object: Box<JSItem>
    },

    Array {
        items: Vec<JSItem>,
        properties: HashMap<String, JSItem>
    },

    Variable {
        mutable: bool,
        value: Expression
    },

    Function {
        mutable: bool,
        params: Vec<Tok>,
        properties: HashMap<String, JSItem>,
        body: Vec<JSItem>
    },

    BcFunction {
        start: usize,
        params: Vec<String>
    },

    ReturnJump {
        to: usize
    }
}

impl Display for JSItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            JSItem::Std { params: _, func } => {
                match func {
                    StdFun::ConsoleLog => write!(f, "f log(){{ [native code] }}"),
                    StdFun::ObjectKeys => write!(f, "f keys(){{ [native code] }}"),
                    StdFun::FunctionApply => write!(f, "f apply(){{ [native code] }}"),
                    StdFun::ArrayMap => write!(f, "f map(){{ [native code] }}"),
                    StdFun::ArrayConstructor => write!(f, "f Array(){{ [native code] }}"),
                    StdFun::ArrayPush => write!(f, "f push(){{ [native code] }}")
                }
            }
            JSItem::St { statement:_ } => {
                write!(f, "statement")
            }
            JSItem::Number {value} => {
                write!(f, "{}", value)
            }
            JSItem::String {value} => {
                write!(f, "{}", value)
            }
            JSItem::Variable { mutable: _, value } => {
                match value {
                    Expression::String {value} => {
                        write!(f, "{}", value)
                    }
                    Expression::Number { value } => {
                        write!(f, "{}", value)
                    }
                    Expression::Literal { value } => {
                        write!(f, "{}", value)
                    }
                    Expression::Null => {
                        write!(f, "null")
                    }
                    Expression::True => {
                        write!(f, "true")
                    }
                    Expression::False => {
                        write!(f, "false")
                    }
                    _ => write!(f, "")
                }
            }
            _ => {
                write!(f, "")
            }
        }
    }
}
