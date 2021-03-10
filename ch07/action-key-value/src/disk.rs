use libactionkeyvalue::ActionKV;
use std::collections::HashMap;
use std::env;
use std::path;

#[cfg(target_os = "windows")]
const USAGE: &'static str = "
Usage:
  disk.exe FILE get KEY
  disk.exe FILE delete KEY
  disk.exe FILE insert KEY VALUE
  disk.exe FILE update KEY VALUE
";

#[cfg(not(target_os = "windows"))]
const USAGE: &str = "
Usage:
  disk FILE get KEY
  disk FILE delete KEY
  disk FILE insert KEY VALUE
  disk FILE update KEY VALUE
";

type ByteStr = [u8];
type ByteString = Vec<u8>;

fn store_index_on_disk(akv: &mut ActionKV, index_key: &ByteStr) {
    akv.index.remove(index_key);

    let index_as_bytes = bincode::serialize(&akv.index).unwrap();
    akv.index = HashMap::new();
    akv.insert(index_key, &index_as_bytes).unwrap();
}

fn main() {
    const INDEX_KEY: &ByteStr = b"+index";

    let args: Vec<String> = env::args().collect();
    let fname = args.get(1).expect(&USAGE);
    let action = args.get(2).expect(&USAGE).as_ref();
    let key: &ByteStr = args.get(3).expect(&USAGE).as_ref();
    let maybe_value = args.get(4);

    let path = path::Path::new(&fname);
    let mut akv = ActionKV::open(path).expect("unable to open file");
    akv.load().expect("unable to load data");
    store_index_on_disk(&mut akv, INDEX_KEY);

    match action {
        "get" => {
            let index_as_bytes = akv.get(&INDEX_KEY).unwrap().unwrap();
            let index: HashMap<ByteString, u64> = bincode::deserialize(&index_as_bytes).unwrap();

            match index.get(key) {
                None => eprintln!("{:?} not found", key),
                Some(value) => println!("{:?}", value),
            }
        }
        "delete" => akv.delete(key).unwrap(),
        "insert" => {
            let value = maybe_value.expect(&USAGE).as_ref(); // an update that could be added for compatibility with Rust's HashMap would be for insert to return the old value, if it exists
            akv.insert(key, value).unwrap()
        }
        "update" => {
            let value = maybe_value.expect(&USAGE).as_ref();
            akv.update(key, value).unwrap()
        }
        _ => eprintln!("{}", &USAGE),
    }
}
