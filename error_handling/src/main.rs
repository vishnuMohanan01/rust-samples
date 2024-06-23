use std::fs::{self, File};
use std::io;

fn main() {
    match File::open("hello.txt") {
        Ok(f) => f,
        Err(error) => {
            match error.kind() {
                io::ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating file: {:?}", e)
                },
                other_error => panic!("Problem opening the file {:?}", other_error)
            }
        }
    };

    File::open("hey.txt").unwrap_or_else(|error| {
        if error.kind() == io::ErrorKind::NotFound {
            File::create("hey.txt").unwrap_or_else(|e| {
                panic!("Problem creating file: {:?}", e);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    File::open("hello.txt").unwrap(); // Throws error if failed
    File::open("hey.txt").expect("failed to open file");

    let s = read_str_from_file();
    println!("The string from file is {:?}", s.unwrap());
}

fn read_str_from_file() -> Result<String, io::Error> {
    fs::read_to_string("string.txt")
}
