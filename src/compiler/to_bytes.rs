use crate::compiler::op_codes::Op;
use bytebuffer::ByteBuffer;

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
                buffer.write_u8(0x0);
            }
            Op::Store { name } => {
                buffer.write_u8(0x9);
                buffer.write_string(&name);
                buffer.write_u8(0x0);
            }
            Op::Load { name } => {
                buffer.write_u8(0xa);
                buffer.write_string(&name);
                buffer.write_u8(0x0);
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
                buffer.write_u8(to as u8);
            }
            Op::JumpAbsolute { to } => {
                buffer.write_u8(0x10);
                buffer.write_u8(to as u8);
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
        }
    }
    return buffer.to_bytes();
}