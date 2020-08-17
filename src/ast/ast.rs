#[derive(PartialEq)]
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
    None,
    True, // The literal 'True'.
    False, // The literal 'False'.
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