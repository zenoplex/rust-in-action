use std::mem::transmute;

fn main() {
  let a: f32 = 42.42;
  let frankentype: u32 = unsafe { transmute(a) };
  println!("u32: {}", frankentype);

  println!("binary format: {:032b}", frankentype);

  let b: f32 = unsafe { transmute(frankentype) };
  println!("f32: {}", b);
}
