#[derive(Debug)]
struct User <'a> {
    name: &'a str
}


fn main() {
    println!("Hello, world!");
    let name = String::from("Parth");
    let user1 = User {
        name: &name
    };
    println!("{:?}", user1);
}
