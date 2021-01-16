// To enable type comparison
#[derive(PartialEq)]
struct Hostname(String);

fn main() {
    let ordinary_string = String::from("localhost");
    let host = Hostname(ordinary_string.clone());

    // This will never be true
    if host == ordinary_string {
        println!("huh?");
    };
}
