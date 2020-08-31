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
    String {
        value: String
    },
    Identifier {
        name: String
    },
    Literal {
        value: String
    },
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
    }
}

/// An operator for a binary operation (an operation with two operands).
#[derive(Debug, PartialEq)]
pub enum Operator {
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
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    //temporary non filled in statement
    None,

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
        params: Vec<Tok>,
        body: Vec<JSItem>
    }
}

#[derive(Debug, PartialEq)]
pub enum JSItem {
    Ex {
        expression: Box<Expression>
    },

    St {
        statement: Box<Statement>
    }
}