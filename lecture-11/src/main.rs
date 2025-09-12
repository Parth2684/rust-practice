use std::fs;

fn main() {
    let res = read_from_a_file("input.txt".to_string());
    println!("{}", res);

    let first_a = find_first_a("hi therea bro".to_string());
    match first_a {
        Option::None => println!("No a found in the string"),
        Option::Some(index) => println!("a found at index {}", index)
    }
}


fn read_from_a_file(file_name: String) -> String {
    let res = fs::read_to_string(file_name);

    match res {
        Result::Ok(str) => str,
        Result::Err(err) => err.to_string() 
    }
}

fn find_first_a(str: String) -> Option<usize> {
    for (index, charachter) in str.chars().enumerate() {
        if charachter == 'a' {
            return Some(index);
        }
    }
    return None
}