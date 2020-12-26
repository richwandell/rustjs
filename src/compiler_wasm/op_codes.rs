#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Inst {
    Unreachable, // 0x00
    Nop, // 0x01
    Block, // 0x02
    Loop, // 0x03
    If, // 0x04
    Else, // 0x05
    End, // 0x0b
    Br, // 0x0c
    BrIf, // 0x0d
    BrTable, // 0x0e
    Return, // 0x0f
    Call {
        x: u8
    }, // 0x10
    CallIndirect {
        x: u8
    }, // 0x11
    Drop, // 0x1a
    Select, // 0x1b
    LocalGet {
        x: u8
    }, // 0x20
    LocalSet {
        x: u8
    }, // 0x21
    LocalTee {
        x: u8
    }, // 0x22
    GlobalGet {
        x: u8
    }, // 0x23
    GlobalSet {
        x: u8
    }, // 0x24
    I32Load, // 0x28
    I64Load, // 0x29
    F32Load, // 0x2a
    F64Load, // 0x2b
    I32Load8s, // 0x2c
    I23Load8u, // 0x2d
    F32Store, // 0x38
    F64Store, // 0x39
    MemSize, // 0x34
    MemGrow, // 0x40
    F32Const, // 0x43
    F64Const, // 0x44
    F64Eq, // 0x61
    F64Ne, // 0x62
    F64Lt, // 0x63
    F64Gt, // 0x64
}