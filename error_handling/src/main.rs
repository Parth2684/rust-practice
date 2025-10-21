use std::{fs::File, io::{ErrorKind}};

fn main() {
    // let vec: Vec<i8> = Vec::new();
    // let el = vec[1];
    // println!("{el}"); //unrecoverable error thread panics and exits the process if the main thread is paniced
    let hi_there = File::open("./hithere.txt");
    match hi_there {
        Ok(file) => file, 
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create("./hithere.txt") {
                Ok(file_created) => file_created,
                Err(e) => panic!("{e}")
            }
            _ => panic!("unknown err")
        }
    };
}