// This code does not work for &str
// fn is_strong(password: String) -> bool {
//   password.len() > 5;
// }

fn is_strong<T: AsRef<str>>(password: T) -> bool {
  password.as_ref().len() > 5
}


fn main() {
    let str = "abcdef";
  is_strong(str);

  let string = String::from("abcdef");
  is_strong(string);
}
