fn main() {
    let mut n_nonzero = 0;

    for i in 0..10000 {
        // Convert to raw pointer
        let pointer = i as *const u8;
        // Dereference the pointer
        let byte_at_address = unsafe { *pointer };

        if byte_at_address != 0 {
            n_nonzero += 1;
        }

        println!("non-zero bytes in memory: {}", n_nonzero);
    }
}
