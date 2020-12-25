use std::fs::File;
use std::io::Write;

mod read_bytes;

#[test]
fn write_wasm() {
    let mut bytes = vec![];

    let len_sig: i16 = 40;


    if len_sig < 256 {
        bytes.push(0x01);
        bytes.push(len_sig as i8);
    } else if len_sig < 65536 {
        bytes.push(0x02);
        bytes.push(len_sig as i8);
        bytes.push((len_sig >> 8) as i8);
    }

    println!("{}", "hi");
}