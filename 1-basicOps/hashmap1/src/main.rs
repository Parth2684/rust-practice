use std::collections::HashMap;

#[derive(Debug)]
struct User {
    name: String,
    age: i32
}

fn main() {
    println!("Hello, world!");

    let mut input_vec: Vec<User> = vec![User {name: String::from("Parth"), age: 21}];
    input_vec.push(User {
        name: String::from("Bhosle"), age: 22
    });
    input_vec.push(User { name: String::from("Bhosle"), age: 21 });
    println!("{:?}", input_vec);
    let hash = group_values_by_keys(input_vec);
    println!("{:?}", hash);
}


fn group_values_by_keys(users: Vec<User>) -> HashMap<String, i32> {
    let mut hash:HashMap<String, i32> = HashMap::new();
    for user in users {
        hash.insert(user.name, user.age);
    }
    hash
}

//