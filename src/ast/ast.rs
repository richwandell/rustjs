use crate::lexer::js_token::Tok;

#[derive(Debug, PartialEq)]
pub enum Expression {
    Binop {
        a: Box<Expression>,
        op: Operator,
        b: Box<Expression>,
    },
    // A numeric literal.
    Number {
        value: f64,
    },
    Identifier {
        name: String
    },
    None,
    True, // The literal 'True'.
    False, // The literal 'False'.
    CallExpression {
        func: Box<Statement>
    }
}

/// An operator for a binary operation (an operation with two operands).
#[derive(Debug, PartialEq)]
pub enum Operator {
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
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Break,

    Continue,

    Return { value: Option<Expression> },

    /// Variable assignment. Note that we can assign to multiple targets.
    Assign {
        targets: Box<Expression>,
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
        params: Vec<Box<Tok>>,
        body: Vec<Box<Expression>>
    }
}