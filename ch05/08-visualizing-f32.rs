// use std::mem::transmute;

const BIAS: i32 = 127;
// use std::f32::RADIX;
const RADIX: f32 = 2.0;

fn deconstruct_f32(n: f32) -> (u32, u32, u32) {
  // 0 00000000 00000000000000000000000 sign/exponent/mantissa
  // let n_: u32 = unsafe { transmute(n) };
  let n_: u32 = n.to_bits();
  let sign = (n_ >> 31) & 1; // strip 31 bits by shifting them and leave sign bit
  let exponent = (n_ >> 23) & 0xff; // strip 23 unwanted bits away and filter with 255(8 bit)
  let fraction = 0b00000000_01111111_11111111_11111111 & n_; // only retain the 23 "`least significant`" bits via a mask

  (sign, exponent, fraction)
}

fn decode_f32_parts(sign: u32, exponent: u32, fraction: u32) -> (f32, f32, f32) {
  let signed_1 = (-1.0_f32).powf(sign as f32); // convert sign bit to 1.0 or -1.0
  let exponent = (exponent as i32) - BIAS;
  let exponent = RADIX.powf(exponent as f32);

  let mut mantissa: f32 = 1.0;
  for i in 0..22 {
    let one_at_bit_i = 1 << i; // create mask for single bit in mantissa
    if (one_at_bit_i & fraction) != 0 {
      mantissa += 2_f32.powf((i as f32) - 23.0); // Decimal value of the bit is 2e(1 - 23)
    }
  }

  (signed_1, exponent, mantissa)
}

fn f32_from_parts(sign: f32, exponent: f32, mantissa: f32) -> f32 {
  sign * exponent * mantissa
}

fn main() {
  let n: f32 = 42.42;
  let (signbit, exponent, fraction) = deconstruct_f32(n);
  let (sign, exponent, mantissa) = decode_f32_parts(signbit, exponent, fraction);
  let reconstituted_n = f32_from_parts(sign, exponent, mantissa);

  println!("original: {:b}", n.to_bits());
  println!(
    "{} -> [sign: {}, exponent: {}, mantissa: {:?}] -> {}",
    n, signbit, exponent, mantissa, reconstituted_n
  );
}
