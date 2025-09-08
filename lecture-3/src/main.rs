fn main() {
//  compound types
    // tuples
    let tup = (500, "hi there", 'c', true);
    println!("{}", tup.0);
    println!("{}", tup.1);
    println!("{}", tup.2);
    println!("{}", tup.3);

    let (w, x, y, z) = tup;
    println!("{}", w);
    println!("{}", x);
    println!("{}", y);
    println!("{}", z);

    // array
    let mut array = [1, 2, 3];
    println!("{}", array[0]);
    println!("{}", array[1]);
    println!("{}", array[2]);

    array[0] = 0;
    println!("{}", array[0]);

    // vectors
    let mut nums = vec![1, 2, 3]; // macro
    nums.push(4);
    println!("{:?}", nums);

    nums.pop();
    println!("{:?}", nums);

    let mut vec = Vec::<i32>::with_capacity(3);
    vec.push(1);
    vec.push(2);
    vec.push(3);

    vec.reverse();
    println!("{:?}", vec);

    let v: Vec<i32> = (0..5).collect();
    println!("{:?}", v);

    let sv: &[i32] = &v[2..4];
    println!("{:?}", sv);
}

