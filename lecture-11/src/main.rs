use std::{fs::read_to_string, io::Error};
use chrono::{ Utc, Local };

fn main() {
    let res = read_from_a_file("input.txt".to_string());
    match res {
        Result::Ok(str) => println!("{}", str),
        Result::Err(str) => println!("Error reading file: {}", str)
    }

    let first_a = find_first_a("hi therea bro".to_string());
    match first_a {
        Option::None => println!("No a found in the string"),
        Option::Some(index) => println!("a found at index {}", index)
    }

    let now = Utc::now();
    println!("Utc time {}", now);

    let formated = now.format("%Y-%m-%d %H:%M:%S");
    println!("formated time {}", formated);

    let local = Local::now();
    let local_formatted = local.format("%Y-%m-%d %H:%M:%S");
    println!("{}", local_formatted);
}


fn read_from_a_file(file_name: String) -> Result<String, Error> {
    read_to_string(file_name)
}

fn find_first_a(str: String) -> Option<usize> {
    for (index, charachter) in str.chars().enumerate() {
        if charachter == 'a' {
            return Some(index);
        }
    }
    return None
}