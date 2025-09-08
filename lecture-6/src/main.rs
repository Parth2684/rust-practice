fn main() {
    let mut num = 0;
    'counter: loop {
        println!("{}", num);
        num+=1;
        let mut decreasing = 5;
        loop {
            println!("{}", decreasing);
            decreasing-=1;
            if decreasing == 0  {
                break;
            }
            if num == 5 {
                break 'counter;
            }
        }
    }

    println!("while loop start from here");

    let mut while_loop = 0;
    while while_loop <= 5 {
        println!("{}", while_loop);
        while_loop += 1;
    }

    println!("For loop for vec");
    let vec: Vec<i8> = (0..10).collect();
    for element in vec {
        println!("{}", element);
    }


    for elem in (0..9).rev() {
        println!("{}", elem);
    }
}
