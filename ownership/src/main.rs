fn main() {
    let _var = 1; // stack
    let mut s = "hello".to_string(); //heap
    s.push_str(", world");
    // ownership
    let x = vec![s.clone()];
    println!("{}", s);
    println!("{:?}", x);
    let num1 = 1;
    let num2 = num1;
    println!("{} {}", num1, num2);

    let mut str = String::from("Hello");
    change_string(&mut str);
    println!("{}", str);
}

fn change_string(some_string: &mut String) {
    some_string.push_str(", world");
}
