#![allow(const_item_mutation)]

use std::io::prelude::*;
use std::io;

const BYTES_PER_LINE: usize = 16;
const INPUT: &'static [u8] = br#"
fn main() {
    println!("Hello, world!");
}
"#;

fn main() -> io::Result<()> {
    let mut buffer: Vec<u8> = vec![];
    // ? on end passes call stack up
    INPUT.read_to_end(&mut buffer)?;    

    let mut position_in_input = 0;
    for line in buffer.chunks(BYTES_PER_LINE) {
        // :08x padded zeros
        print!("[0x{:08x}] ", position_in_input);

        for byte in line {
            print!("{:02x} ", byte);
        }
        println!();
        position_in_input += BYTES_PER_LINE;
    }

    Ok(())
}
