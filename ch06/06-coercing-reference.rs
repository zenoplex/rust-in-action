fn main() {
  let a: i64 = 42;
  let a_pointer = &a as *const i64;

  println!("a: {} ({:p})", a, a_pointer);
}
