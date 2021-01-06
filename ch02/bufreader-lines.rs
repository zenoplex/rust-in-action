use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let f = File::open("readme.md").unwrap();
    let reader = BufReader::new(f);

    // Each Result from reader.lines will not have newline byte
    for line_ in reader.lines() {
        let line = line_.unwrap();
        println!("{} ({} bytes long)", line, line.len());
    }
}
