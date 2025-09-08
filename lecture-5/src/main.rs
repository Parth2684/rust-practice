fn main() {
    print_function("Hi there people");
}


fn print_function(phrase: &str) {
    println!("{}", phrase);
    let ans = greatest_common_divisor(1245441516516, 3152656541020);
    println!("{}", ans);
    println!("{}", flag(true))
}

fn greatest_common_divisor (mut num1: u128, mut num2: u128) -> u128{
    while num1 != 0 {
        if num1 < num2 {
            let c = num1;
            num1 = num2;
            num2 = c;
        }
        num1 %= num2;
    }
    num2
}

fn flag (input: bool) -> bool {
    if input == true {
        true
    }else {
        false
    }
}