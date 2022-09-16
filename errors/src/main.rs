use std::fs::File;
use std::io::{ self, Read, ErrorKind };

fn matching_on_different_errors() {
    let greeting_file = File::open("hello.txt");
    let _greeting_file = match greeting_file {
        Ok(file) => file,
        // create and return file handler if it does not exist.
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file_handler) => file_handler,
                Err(e) => panic!("Problem creating the file {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            },
        },
    };
}

fn alternative_to_using_match() {
    let _greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error)
        }
    });
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e)
    }
}

fn main() {
    matching_on_different_errors();
    alternative_to_using_match();
    read_username_from_file();
}
