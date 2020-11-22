use crate::compiler::to_bytes::from_bytes;
use crate::compiler::op_codes::Op;

#[test]
fn test_from_bytes_math_1() {
    let mut bytes = vec![
        7, 63, 240, 0, 0, 0, 0, 0, 0, 7, 64, 0, 0, 0, 0, 0, 0, 0, 2, 7, 64, 8, 0, 0, 0, 0, 0, 0, 2
    ];

    let ops = from_bytes(bytes);

    assert_eq!(ops, vec![
        Op::LoadNumConst {
            value: 1.
        },
        Op::LoadNumConst {
            value: 2.
        },
        Op::Add,
        Op::LoadNumConst {
            value: 3.
        },
        Op::Add
    ]);
}

#[test]
fn test_console_log() {
    let mut bytes = vec![10, 0, 0, 0, 7, 99, 111, 110, 115, 111, 108, 101, 19, 0, 0, 0, 3, 108,
                         111, 103, 8, 0, 0, 0, 2, 104, 105, 12, 1];

    let ops = from_bytes(bytes);

    assert_eq!(ops, vec![
        Op::Load {
            name: "console".to_string()
        },
        Op::LoadProp {
            name: "log".to_string()
        },
        Op::LoadStrConst {
            value: "hi".to_string()
        },
        Op::Call {args: 1}
    ]);
}