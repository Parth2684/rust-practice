fn main() {
    // scalar data types

    // int
    let x: i8 = 10;
    println!("Value of i8 x is {}", x);

    let y: u8 = 10;
    println!("Value of u8 y is {}", y);

    let decimal = 0_255;
    let hex = 0xff;
    let octal = 0o377;
    let binary = 0b1111_1111;

    println!("{}", decimal);
    println!("{}", hex);
    println!("{}", octal);
    println!("{}", binary);

    let byte = b'A';
    println!("{}", byte);

    // floats
    let z = 2.0;
    let another_float :f64 = 1.0;
    println!("{}, {}", z, another_float);

    // boolean
    let t= true;
    let f: bool = false;
    println!("{}, {}", t, f);

    // charachter
    let c = 'c';
    println!("{}", c);

    // arithmetic operators
    let a = 10;
    let b = 4;

    println!("Remainder, {}", a%b)
}
