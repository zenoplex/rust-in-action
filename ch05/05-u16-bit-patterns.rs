fn main() {
  let zero: u16 = 0b0000_0000_0000_0000;
  let one: u16 = 0b0000_0000_0000_0001;
  let two: u16 = 0b0000_0000_0000_0010;
  let sixty_five_thousand_533: u16 = 0b1111_1111_1111_1101;
  let sixty_five_thousand_534: u16 = 0b1111_1111_1111_1110;
  let sixty_five_thousand_535: u16 = 0b1111_1111_1111_1111;

  print!("{}, {}, {}, ...", zero, one, two);
  println!(
    "{}, {}, {}",
    sixty_five_thousand_533, sixty_five_thousand_534, sixty_five_thousand_535
  );
}
