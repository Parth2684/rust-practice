fn main() {
    let mut vec: Vec<i8> = Vec::new();
    vec.push(1);
    vec.push(3);
    vec.push(5);
    vec.push(7);
    vec.push(9);
    
    let vec2: Vec<i8> = vec.iter().map(|x| x*10).collect();
    println!("{vec2:?}");
}
