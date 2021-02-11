use std::mem;

fn main() {
  let big_endian: [u8; 4] = [0xAA, 0xBB, 0xCC, 0xDD];
  let little_endian: [u8; 4] = [0xDD, 0xCC, 0xBB, 0xAA];

  // transmute function tells compiler to interpret its argument as the type on the variable declaration
  let a: i32 = unsafe { mem::transmute(big_endian) };
  let b: i32 = unsafe { mem::transmute(little_endian) };

  println!("{} vs {}", a, b);
}
