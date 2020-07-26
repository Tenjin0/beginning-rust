use std::fs::File;
use std::io;
use std::fs;
use std::io::Read;
use std::io::ErrorKind;

pub fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;

    let mut s = String::new();

    f.read_to_string(&mut s)?;
    Ok(s)
}
    

#[allow(dead_code)]
pub fn read_username_from_file2() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

#[allow(dead_code)]
pub fn main() {
    // let _f = File::open("hello.txt").expect("Problem opening the file");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file {:?}", other_error)
    //         }
    //     }
    // };

    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file {:?}", error)
            })
        } else {
            panic!("Problem opening the file {:?}", error)
        }
    });

    println!("{:?}", f);
}
