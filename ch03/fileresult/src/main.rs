use rand::Rng;

fn one_in(n: u8) -> bool {
    rand::thread_rng().gen_bool(1.0 / n as f64)
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
        }
    }

    fn new_with_data(name: &str, data: Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }

    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        let mut tmp = self.data.clone();
        let read_length = tmp.len();
        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        Ok(read_length)
    }
}

fn open(f: File) -> Result<File, String> {
    if one_in(3) {
        let err_msg = String::from("Permission denied");
        return Err(err_msg);
    }
    Ok(f)
}

fn close(f: File) -> Result<File, String> {
    if one_in(5) {
        let err_msg = String::from("Interrupted by signal");
        return Err(err_msg);
    }
    Ok(f)
}

fn main() {
    let data = vec![114, 117, 115, 116, 33];
    let mut file = File::new_with_data("4.txt", data);
    let mut buffer: Vec<u8> = vec![];

    file = open(file).unwrap();
    let length = file.read(&mut buffer).unwrap();
    file = close(file).unwrap();
    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", file);
    println!("{} is {} bytes long", &file.name, length);
    println!("{}", text);
}
