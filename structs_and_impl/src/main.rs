struct User {
    active: bool,
    username: String,
    sign_in_count: u32
}

struct Coordinates(i32, i32, i32, i32);

struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area (&self) -> u32 {
        self.height * self.width
    }
}


fn main() {
    let user1 = User {
        active: true,
        username: String::from("Parth"),
        sign_in_count: 0
    };
    println!("{}", user1.username);

    let _cords = Coordinates(1, 2, 3, 4);
    let mut rectangle = Rectangle {
        width: 5,
        height: 10
    };

    rectangle.width = 10;


    println!("{}", rectangle.area())
}
