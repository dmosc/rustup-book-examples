use std::{
    fs::File,
    io::{self, ErrorKind, Read},
};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)

    // Or use:
    //
    // use std::fs;
    // fs::read_to_string("...")
}

fn main() {
    let file = File::open("src/text.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("src/text.txt").expect("problem creating the file {error}")
        } else {
            panic!("problem opening the file {:?}", error);
        }
    });
    println!("{:?}", file);
    read_username_from_file().expect("The unexpected");
}
