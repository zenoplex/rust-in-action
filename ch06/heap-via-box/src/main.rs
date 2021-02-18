use std::mem::drop;

fn main() {
    // Allocate values to heap
    let a = Box::new(1);
    let b = Box::new(1);
    let c = Box::new(1);

    let result1 = *a + *b + *c;

    // free memory. a.drop cannot be called explicitly
    drop(a);

    let d = Box::new(1);
    let result2 = *b + *c + *d;
    println!("{}, {}", result1, result2);
}
