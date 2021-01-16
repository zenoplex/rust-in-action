// Relax comipler warnings
#![allow(unused_variables)]

type File = String;

fn open(f: &mut File) -> bool {
    true
}

fn close(f: &mut File) -> bool {
    true
}

#[allow(dead_code)]
fn read(f: File, save_to: Vec<u8>) -> ! {
    // Macro to crash the program if encountered
    unimplemented!()
}

fn main() {
    let mut f1 = File::from("f1.txt");
    open(&mut f1);
    // read(f1, vec![]);
    close(&mut f1);
}
