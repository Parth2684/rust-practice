use std::ops::Add;
#[derive(Debug)]
struct Points <T> {
    x: T,
    y: T
}

impl <T> Add for Points<T>
    where
        T: Add<Output = T> {
            type Output = Self;
            fn add(self, rhs: Self) -> Self {
                Points { x:self.x + rhs.x, y: rhs.y + self.y }
            }
        }


fn main() {
    let sum = Points{x: 1, y: 2 } + Points{x: 2, y: 5};
    println!("{sum:?}");
}
