use libactionkeyvalue::ActionKV;
use std::env;
use std::path;

#[cfg(target_os = "windows")]
const USAGE: &'static str = "
Usage:
  mem.exe FILE get KEY
  mem.exe FILE delete KEY
  mem.exe FILE insert KEY VALUE
  mem.exe FILE update KEY VALUE
";

#[cfg(not(target_os = "windows"))]
const USAGE: &str = "
Usage:
  mem FILE get KEY
  mem FILE delete KEY
  mem FILE insert KEY VALUE
  mem FILE update KEY VALUE
";

fn main() {
    let args: Vec<String> = env::args().collect();
    let fname = args.get(1).expect(&USAGE);
    let action = args.get(2).expect(&USAGE).as_ref();
    let key = args.get(3).expect(&USAGE).as_ref();
    let maybe_value = args.get(4);

    let path = path::Path::new(&fname);
    let mut a = ActionKV::open(path).expect("unable to open file");
    a.load().expect("unable to load data");

    match action {
        "get" => {
            match a.get(key).unwrap() {
                None => eprintln!("{:?} not found", key),
                Some(value) => println!("{:?}", value), // needs to use Debug as [u8] is arbitrary bytes
            }
        }
        "delete" => a.delete(key).unwrap(),
        "insert" => {
            let value = maybe_value.expect(&USAGE).as_ref(); // an update that could be added for compatibility with Rust's HashMap would be for insert to return the old value, if it exists
            a.insert(key, value).unwrap()
        }
        "update" => {
            let value = maybe_value.expect(&USAGE).as_ref();
            a.update(key, value).unwrap()
        }
        _ => eprintln!("{}", &USAGE),
    }
}
