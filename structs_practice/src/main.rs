#[derive(Debug)]
struct Car {
    mpg: i32,
    color: String,
    top_speed: i32
}

impl Car {
    fn set_mpg(&mut self, mpg: i32 ) {
        self.mpg = mpg;
    }
    fn set_color (&'_ mut self, color: String) {
        self.color = color;
    }
    fn set_top_speed(&mut self, speed:i32) {
        self.top_speed = speed;
    }
}

fn main() {
    let mut car1 = Car {
        mpg: 0,
        color: String::from("Red"),
        top_speed: 32
    };
    
    car1.set_mpg(45);
    car1.set_top_speed(200);
    car1.set_color(String::from("Blue"));
    println!("{car1:?}");
    
}
