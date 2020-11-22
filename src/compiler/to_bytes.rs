use crate::compiler::op_codes::Op;
use bytebuffer::ByteBuffer;
use std::io::Bytes;

pub(crate) fn to_bytes(ops: Vec<Op>) -> Vec<u8> {
    let mut buffer = ByteBuffer::new();

    for op in ops {
        match op {
            Op::Return => {
                buffer.write_u8(0x1)
            }
            Op::Add => {
                buffer.write_u8(0x2)
            }
            Op::Sub => {
                buffer.write_u8(0x3)
            }
            Op::Div => {
                buffer.write_u8(0x4)
            }
            Op::Mul => {
                buffer.write_u8(0x5)
            }
            Op::Less => {
                buffer.write_u8(0x6)
            }
            Op::LoadNumConst { value } => {
                buffer.write_u8(0x7);
                buffer.write_f64(value);
            }
            Op::LoadStrConst { value } => {
                buffer.write_u8(0x8);
                buffer.write_string(&value);
            }
            Op::Store { name } => {
                buffer.write_u8(0x9);
                buffer.write_string(&name);
            }
            Op::Load { name } => {
                buffer.write_u8(0xa);
                buffer.write_string(&name);
            }
            Op::LoadMember => {
                buffer.write_u8(0xb)
            }
            Op::Call { args } => {
                buffer.write_u8(0xc);
                buffer.write_u8(args as u8);
            }
            Op::PopTop => {
                buffer.write_u8(0xd)
            }
            Op::SetupLoop => {
                buffer.write_u8(0xe)
            }
            Op::PopJumpIfFalse { to } => {
                buffer.write_u8(0xf);
                buffer.write_u64(to as u64);
            }
            Op::JumpAbsolute { to } => {
                buffer.write_u8(0x10);
                buffer.write_u64(to as u64);
            }
            Op::PopBlock => {
                buffer.write_u8(0x11)
            }
            Op::InplaceAdd => {
                buffer.write_u8(0x12)
            }
            Op::LoadProp { name } => {
                buffer.write_u8(0x13);
                buffer.write_string(&name);
            }
            Op::DeclareFunc { start, end, mutable, params, name } => {
                buffer.write_u8(0x14);
                buffer.write_u64(start as u64);
                buffer.write_u64(end as u64);
                buffer.write_bit(mutable);
                buffer.write_u64(params.len() as u64);
                for param in params {
                    buffer.write_string(&param);
                }
                buffer.write_string(&name);
            }
        }
    }
    return buffer.to_bytes();
}

fn next_str(i: usize, bytes: &Vec<u8>) -> (String, usize) {
    let num: [u8; 4] = [
        bytes.get(i + 1).unwrap().clone(),
        bytes.get(i + 2).unwrap().clone(),
        bytes.get(i + 3).unwrap().clone(),
        bytes.get(i + 4).unwrap().clone()
    ];
    let n_bytes = u32::from_be_bytes(num);
    let mut string_bytes = vec![];

    for n in 0..n_bytes {
        string_bytes.push(bytes.get(1 + i + 4 + n as usize).unwrap().clone());
    }
    let string = std::str::from_utf8(&string_bytes).unwrap();
    return (string.to_string(), 4 + n_bytes as usize);
}

pub(crate) fn from_bytes(bytes: Vec<u8>) -> Vec<Op> {
    let mut ops = vec![];


    let mut i = 0;

    while i < bytes.len() {
        let byte = bytes.get(i).unwrap();

        if *byte == 0x1 as u8 {
            ops.push(Op::Return);
        }
        else if *byte == 0x2 as u8 {
            ops.push(Op::Add);
        }
        else if *byte == 0x3 as u8 {
            ops.push(Op::Sub);
        }
        else if *byte == 0x4 as u8 {
            ops.push(Op::Div);
        }
        else if *byte == 0x5 as u8 {
            ops.push(Op::Mul);
        }
        else if *byte == 0x6 as u8 {
            ops.push(Op::Less);
        }
        else if *byte == 0x7 as u8 {
            let num: [u8; 8]  = [
                bytes.get(i + 1).unwrap().clone(),
                bytes.get(i + 2).unwrap().clone(),
                bytes.get(i + 3).unwrap().clone(),
                bytes.get(i + 4).unwrap().clone(),
                bytes.get(i + 5).unwrap().clone(),
                bytes.get(i + 6).unwrap().clone(),
                bytes.get(i + 7).unwrap().clone(),
                bytes.get(i + 8).unwrap().clone(),
            ];
            let float_num = f64::from_be_bytes(num);
            ops.push(Op::LoadNumConst { value: float_num });
            i += 8;
        }
        else if *byte == 0x8 as u8 {
            let vals = next_str(i, &bytes);
            ops.push(Op::LoadStrConst {value: vals.0});
            i += vals.1;
        }
        else if *byte == 0x9 as u8 {

        }
        else if *byte == 0xa as u8 {
            let vals = next_str(i, &bytes);
            ops.push(Op::Load {name: vals.0});
            i += vals.1;
        }
        else if *byte == 0xb as u8 {

        }
        else if *byte == 0xc as u8 {
            ops.push(Op::Call {args: bytes.get(i + 1).unwrap().clone() as i8});
            i += 1;
        }
        else if *byte == 0x10 as u8 {
            let num: [u8; 8]  = [
                bytes.get(i + 1).unwrap().clone(),
                bytes.get(i + 2).unwrap().clone(),
                bytes.get(i + 3).unwrap().clone(),
                bytes.get(i + 4).unwrap().clone(),
                bytes.get(i + 5).unwrap().clone(),
                bytes.get(i + 6).unwrap().clone(),
                bytes.get(i + 7).unwrap().clone(),
                bytes.get(i + 8).unwrap().clone(),
            ];
            let nnum = u64::from_be_bytes(num);
            ops.push(Op::JumpAbsolute { to: nnum as usize })
        }
        else if *byte == 0x13 as u8 {
            let vals = next_str(i, &bytes);
            ops.push(Op::LoadProp {name: vals.0});
            i += vals.1;
        }
        i += 1;
    }

    return ops;
}