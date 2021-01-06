fn main() {
    let three = 0b11; // base 2
    let thirty = 0o36; // base 8
    let three_hundered = 0x12C; // base 16

    println!("base 10: {} {} {}", three, thirty, three_hundered);
    println!("base 2: {:b} {:b} {:b}", three, thirty, three_hundered);
    println!("base 8: {:o} {:o} {:o}", three, thirty, three_hundered);
}
