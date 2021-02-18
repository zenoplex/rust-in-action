fn main() {
  // On stack
  let a: i32 = 40;
  // On heap due to pointer
  let b: Box<i32> = Box::new(60);  

  // To access numerical 60, b needs to be deferenced
  println!("{} + {} = {}", a, b, a + *b);
}
