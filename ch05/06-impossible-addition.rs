// Compiling this code with below code will give wrong result
// rustc -O impossible-addition.rs && ./impossible-addition

// Compiler will warn us but allowing for testing
#![allow(arithmetic_overflow)]

fn main() {
  let (a, b) = (200, 200);
  let c: u8 = a + b;

  println!("200 + 200 = {}", c);
}
