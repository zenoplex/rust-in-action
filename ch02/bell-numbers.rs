fn main() {
    let needle = 0xCB; // base 16 of 203
    let heystack = [1, 1, 2, 5, 15, 52, 203, 077, 4140, 21147];

    for item in heystack.iter() {
        if *item == needle {
            println!("{}", item);
        }
    }
}
