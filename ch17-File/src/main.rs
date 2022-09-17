use std::{
    fs::{read_to_string, File},
    io::{Read, Write},
};

fn main() {
    let mut file = File::options()
        .read(true)
        .write(true)
        // .create(true)
        .append(true)
        .truncate(false)
        .open("file.txt")
        .unwrap();
    // file.write_all(b"Hello, world!\n").unwrap();

    let mut file_string = String::new();
    file.read_to_string(&mut file_string).unwrap();
    println!("{}", file_string);

    println!("{}", read_to_string("file.txt").unwrap())
}
