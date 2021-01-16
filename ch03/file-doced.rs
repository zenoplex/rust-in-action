// run rustdoc file-doced.rs
//! Simulating files one step at a time.

/// Represents a "file", which probably lives on a file system.
#[derive(Debug)]
pub struct File {
    name: String,
    data: Vec<u8>,
}

impl File {
    /// New files are assumed to be empty, but a name is required
    /// ```
    /// let f = File::new("test.txt");
    /// ```
    pub fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
        }
    }

    /// Returns the file's length in bytes
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns the file's name
    pub fn name(&self) -> String {
        self.name.clone()
    }
}

fn main() {
    let file = File::new("test.txt");
    let name = file.name();
    let length = file.len();

    println!("{:?}", file);
    println!("{} is {} bytes long", name, length);
}
