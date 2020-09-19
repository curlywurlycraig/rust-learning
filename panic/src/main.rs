use std::fs::File;
use std::io::prelude::*;

fn main() {
    // Uncommenting the following will cause the program to fail with an error.
    // panic!("oh no");

    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(error) => panic!("Couldn't open the file: {}", error)
    };

    let mut file_contents = String::new();
    f.read_to_string(&mut file_contents).unwrap();

    println!("The file says: {}", file_contents);
}
