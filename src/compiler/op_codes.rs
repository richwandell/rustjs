#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Op {
    Return,
    Add,
    Sub,
    Div,
    Mul,
    Less,
    LoadNumConst { // load num const
        value: f64
    },
    LoadStrConst { // load string const
        value: String
    },
    Store {
        name: String
    },
    Load {
        name: String
    },
    LoadMember,
    Call {
        args: i8
    },
    PopTop,
    SetupLoop,
    PopJumpIfFalse {
        to: usize
    },
    JumpAbsolute {
        to: usize
    },
    PopBlock,
    InplaceAdd
}