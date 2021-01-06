use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let f = File::open("readme.md").unwrap();
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    loop {
        let len = reader.read_line(&mut line).unwrap();
        // end loop if EOF is reached
        if len == 0 {
            break;
        }
        println!("{} ({} bytes long)", line, len);
        // must shrink length back to 0 to prevent from persisting into following iteration
        // This is required because we are reusing line outside of loop
        line.truncate(0);
    }
}
