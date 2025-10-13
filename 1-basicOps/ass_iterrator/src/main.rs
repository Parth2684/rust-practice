fn main() {
    println!("Hello, world!");
    let vec = vec![1,2,3,4];
    let iter1: Vec<i32> = vec.iter().filter(|x| **x % 2 != 0).map(|x| x * 2).collect();
    println!("{:?}", iter1);
}
