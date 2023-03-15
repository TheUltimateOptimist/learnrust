use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) =>  file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    //more concise:
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound{
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    //panic!("crash and burn")
    // let v = vec![1,2,3];
    // v[99];
}

fn last_char_of_first_line(text: &str) -> Option<char>{
    text.lines().next()?.chars().last()
}

use std::error::Error;

fn mainWithError() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt");
    Ok(())
}