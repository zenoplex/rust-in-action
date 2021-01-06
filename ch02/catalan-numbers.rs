fn main() {
    let needle = 0o204; // base 8 of 132
    let heystack = vec![1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862, 16796];

    for item in heystack.into_iter() {
        //  dereferencing is not required anymore (*item)
        if item == needle {
            println!("{}", item);
        }
    }
}
