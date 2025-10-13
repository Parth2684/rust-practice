fn main() {
    println!("{}", find_larger(1, 2));
    println!("{}", find_larger("Niggaz", "Whites"));    
}

fn find_larger<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    }else {
        b
    }
}
