use std::mem::transmute;

fn main() {
  let a: i64 = 42;
  let a_pointer = &a as *const i64;
  let a_address: usize = unsafe {
    transmute(a_pointer)
  };

  println!("a: {} ({:p}...0x{:x})", a, a_pointer, a_address + 7);
}
