use std::fs::File;
use std::io::Read;

#[test]
fn read_bytes() {
    let mut f = File::open("scratch/a.wasm").unwrap();


    let mut buffer = Vec::new();
    // read the whole file
    f.read_to_end(&mut buffer);


    println!("{}", "hi");
}