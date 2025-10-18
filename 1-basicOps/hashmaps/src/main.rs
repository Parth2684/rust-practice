use std::collections::HashMap;


fn main() {
    println!("Hello, world!");
    let mut users = HashMap::new();
    users.insert("Parth".to_string(), 21);
    users.insert("Bhosle".to_string(), 22);
    println!("{:?}", users);

    let user1 = users.get("jbg");
    match user1 {
        Some(age) => println!("{}", age),
        None => println!("user not found")
    }
}
