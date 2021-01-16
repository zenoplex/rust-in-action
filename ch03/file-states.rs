#[derive(Debug, PartialEq)]
enum FileState {
    Open,
    Closed,
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }

    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        if self.state != FileState::Open {
            return Err(String::from("File must be open for reading."));
        }

        let mut tmp = self.data.clone();
        let length = tmp.len();
        save_to.reserve(length);
        save_to.append(&mut tmp);
        Ok(length)
    }
}

fn open(mut f: File) -> Result<File, String> {
    f.state = FileState::Open;
    Ok(f)
}

fn close(mut f: File) -> Result<File, String> {
    f.state = FileState::Closed;
    Ok(f)
}

fn main() {
    let mut file = File::new("test.txt");
    let mut buffer: Vec<u8> = vec![];

    // Check with is_error to prevent panic with unwrap
    if file.read(&mut buffer).is_err() {
        println!("Error checking is working.");
    }

    file = open(file).unwrap();
    let length = file.read(&mut buffer).unwrap();
    file = close(file).unwrap();

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", file);
    println!("{} is {} bytes long", file.name, length);
    println!("{}", text);
}
