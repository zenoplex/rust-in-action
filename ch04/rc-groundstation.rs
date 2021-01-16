use std::rc::Rc;

#[derive(Debug)]
struct GroundStation {}

fn main() {
    // Rc<T> implements Clone and does not allow mutation
    let base = Rc::new(GroundStation {});

    println!("{:?}", base);
}
