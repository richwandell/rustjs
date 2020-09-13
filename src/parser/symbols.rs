use crate::lexer::js_token::Tok;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Expression {
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
        arguments: Vec<Tok>
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
    }
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
pub(crate) enum Statement {
    //temporary non filled in statement
    None,

    Break,

    Continue,

    Return { value: Option<Expression> },

    AssignExpression {
        mutable: bool,
        name: String,
        value: Box<Expression>,
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

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum StdFun {
    ConsoleLog
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum JSItem {
    Number {
        value: f64
    },

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

    Variable {
        mutable: bool,
        value: Expression
    },

    Function {
        mutable: bool,
        params: Vec<Tok>,
        properties: HashMap<String, JSItem>,
        body: Vec<JSItem>
    }
}

impl Display for JSItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            JSItem::Std { params: _, func } => {
                match func {
                    StdFun::ConsoleLog => write!(f, "f log(){{ [native code] }}")
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
