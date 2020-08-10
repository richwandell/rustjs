pub enum Tok {
    Await,
    Break,
    Case,
    Catch,
    Class,
    Const,
    Continue,
    Debugger,
    Default,
    Delete,
    Do,
    Else,
    Enum,
    Export,
    Extends,
    False,
    Finally,
    For,
    Function, // function
    AnonFunction, // () =>
    If,
    Implements,
    Import,
    In,
    InstanceOf,
    Interface,
    Let,
    New,
    Null,
    Package,
    Private,
    Protected,
    Public,
    Return,
    Super,
    Switch,
    Static,
    This,
    Throw,
    Try,
    True,
    TypeOf,
    Var,
    Void,
    While,
    With,
    Yield,

    Name { name: String },
    Float { value: f64 },
    String { value: String },
    StartProgram,
    StartStatement,
    StartExpression,
    EndOfFile,
    Lpar, // (
    Rpar, // )
    Lsqb, // [
    Rsqb, // ]
    Colon, // :
    Comma, // ,
    Semi, // ;
    Plus, // +
    Minus, // -
    Star, // *
    Slash, // \
    Bslash, // /
    Vbar,  // '|'
    Amper, // '&'
    Less, // <
    Greater, // >
    Equal, // =
    Dot, // .
    Percent, // %
    Lbrace, // {
    Rbrace, // }
    EqEqual, // ==
    EqEqEual, // ===
    NotEqual, // !=
    NotDoubleEqual, // !==
    LessEqual, // <=
    GreaterEqual, // >=
    LeftShift, // <<
    RightShift, // >>
    PlusEqual, // +=
    MinusEqual, // -=
    StarEqual, // *=
    RsingleArrow, // ->
    RdoubleArrow // =>
}
