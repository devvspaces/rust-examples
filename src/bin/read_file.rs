use std::fs;
use std::io::{Error, Read};

fn main() {
    let username = read().expect("Failed to read file");
    println!("The username is {username}")
}

fn read() -> Result<String, Error> {
    let mut username = String::new();
    fs::File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}
