// Examples of error handling in Rust

use std::fs::File;
use std::io::{self, Read};

pub fn error_handling_demo() {
    // Result enum
    fn read_username_from_file() -> Result<String, io::Error> {
        let f = File::open("hello.txt");
        
        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };
        
        let mut s = String::new();
        
        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }
    
    // Using the ? operator (shorthand for the match above)
    fn read_username_from_file_short() -> Result<String, io::Error> {
        let mut s = String::new();
        File::open("hello.txt")?.read_to_string(&mut s)?;
        Ok(s)
    }
    
    // Using match with Result
    match read_username_from_file() {
        Ok(username) => println!("Username: {}", username),
        Err(error) => println!("Error reading file: {}", error),
    }
    
    // Unwrap and expect
    // let s = read_username_from_file().unwrap(); // panics if Err
    // let s = read_username_from_file().expect("Failed to read username"); // panics with custom message if Err
    
    // Option and unwrap
    let v = vec![1, 2, 3];
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
}