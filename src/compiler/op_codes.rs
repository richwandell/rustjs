#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Op {
    Return, // 0x1
    Add, // 0x2
    Sub, // 0x3
    Div, // 0x4
    Mul, // 0x5
    Less, //0x6
    LoadNumConst { // 0x7
        value: f64
    },
    LoadStrConst { // 0x8
        value: String
    },
    Store { // 0x9
        name: String
    },
    Load { // 0xa
        name: String
    },
    LoadMember, //0xb
    Call { // 0xc
        args: i8
    },
    PopTop, // 0xd
    SetupLoop, // 0xe
    PopJumpIfFalse { // 0xf
        to: usize
    },
    JumpAbsolute { // 0x10
        to: usize
    },
    PopBlock, // 0x11
    InplaceAdd, // 0x12,
    LoadProp { // 0x13
        name: String
    },
    DeclareFunc { // 0x14
        start: usize,
        end: usize,
        mutable: bool,
        params: Vec<String>,
        name: String
    }
}

