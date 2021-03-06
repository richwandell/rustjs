#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Tok {
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
    EndOfLine,
    EndOfFile,
    Lpar, // (
    Rpar, // )
    Lsqb, // [
    Rsqb, // ]
    Colon, // :
    Comma, // ,
    Semi, // ;
    Plus, // +
    PlusPlus, // ++
    Minus, // -
    Star, // *
    Slash, // \
    Bslash, // /
    BslashEqual, // /=
    BslashBslash, // //
    Vbar,  // '|'
    Amper, // '&'
    AmpAmp, // &&
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
    LeftShiftEqual, // <<=
    RightShiftEqual, // >>=
    RightShiftUnsigned, // >>>
    RightShiftUnsignedEqual, // >>>=
    PlusEqual, // +=
    MinusEqual, // -=
    StarEqual, // *=
    RsingleArrow, // ->
    RdoubleArrow // =>
}
