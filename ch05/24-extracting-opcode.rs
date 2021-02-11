fn extract_rgb(hex: u32) -> [u8; 3] {
  let r = ((hex & 0xFF0000) >> 16) as u8;
  let g = ((hex & 0x00FF00) >> 8) as u8;
  let b = ((hex & 0x0000FF) >> 0) as u8;

  [r, g, b]
}

fn main() {
  let opcode: u16 = 0x71E4;
  let c = (opcode & 0xF000) >> 12;
  let x = (opcode & 0x0F00) >> 8;
  let y = (opcode & 0x00F0) >> 4;
  let d = (opcode & 0x000F) >> 0;

  assert_eq!(c, 0x7);
  assert_eq!(x, 0x1);
  assert_eq!(y, 0xE);
  assert_eq!(d, 0x4);

  let nnn = opcode & 0x0FFF;
  let kk = opcode & 0x00FF;

  assert_eq!(nnn, 0x01E4);
  assert_eq!(kk, 0xE4);

  let [r, g, b] = extract_rgb(0xFFFFFF);
  assert_eq!(r, 255);
  assert_eq!(g, 255);
  assert_eq!(b, 255);

  let [r, g, b] = extract_rgb(0x808080);
  assert_eq!(r, 128);
  assert_eq!(g, 128);
  assert_eq!(b, 128);

  let [r, g, b] = extract_rgb(0x000000);
  assert_eq!(r, 0);
  assert_eq!(g, 0);
  assert_eq!(b, 0);
}
