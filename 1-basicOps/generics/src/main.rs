#[derive(Debug)]
struct _User {
    name: String,
    age: i32
}
fn main() {
    println!("{}", find_larger(1, 2));
    println!("{}", find_larger("Niggaz", "Whites"));
    // println!("{:?}", find_larger(User{name: "Parth".to_string(), age: 32}, User { name: "Karna".to_string(), age: 39 })); gives an error
}

fn find_larger<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    }else {
        b
    }
}
