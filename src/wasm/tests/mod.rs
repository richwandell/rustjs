use std::fs::File;
use std::io::Write;
use crate::wasm::compiler::WasmCompiler;

mod read_bytes;

#[test]
fn write_wasm() {
    let mut comp = WasmCompiler::new();

    comp.add_func(
        vec![0x7f, 0x7f],
        vec![0x7f],
        vec![0x20, 0x00, 0x20, 0x01, 0x6a],
        true,
        "add"
    );

    let output = comp.get_bytes();

    let mut file = File::create("out.wasm").unwrap();
    file.write_all(&comp.output);
    println!("{}", "hi");
}