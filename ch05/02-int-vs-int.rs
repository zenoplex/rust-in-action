fn main() {
  let a: u16 = 50115;
  let b: i16 = -15421;

  // Proof of these two values having same bit pattern
  println!("a: {:016b} {}", a, a);
  println!("b: {:016b} {}", b, b);
}
