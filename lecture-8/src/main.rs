fn main() {
    let mut vec = vec![1, 3, 5, 7];
    
    println!("{:?}", func(&vec));
    vec.push(15);
    println!("{:?}", vec);
    let mut val = 2;
    add_two(val);
    val += 5;
    println!("{}", val)
}

fn func (val: &Vec<i8>) -> bool {
    if val[0] == 1 {
        true
    }else {
        false
    }
}

fn add_two(num1: i8) {
    num1 + 2;
}
