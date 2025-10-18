trait Summary {
    fn summarise(&self) -> String;
}

struct User {
    name: String,
    age: u16
}

impl Summary for User {
    fn summarise(&self) -> String {
        format!("{} is {} years old", self.name, self.age)
    }
}
impl Summary for String {
    fn summarise(&self) -> String {
        format!("Hello world")
    }
}

fn main() {
    println!("Hello, world!");
    let user = User{
        name: "Parth".to_string(),
        age: 39
    };
    let print_statement = user.summarise();
    notify(&print_statement);
    println!("{}", print_statement);
}

fn notify (something: &impl Summary) {
    println!("{}", something.summarise());
}
