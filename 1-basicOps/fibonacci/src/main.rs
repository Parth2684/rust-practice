fn main() {
    println!("Hello, world!");
    for num in fibonacci(3) {
        println!("{}, {}", fibonacci(3).len(),  num);
    }
}


fn fibonacci (end: u32) ->Vec<u128> {
    if end == 0 || end == 1 {
        let result:Vec<u128> = vec![0];
        return result; 
    }else if end == 2 {
        let result: Vec<u128> = vec![0, 1];
        return result;
    }
    let mut result: Vec<u128> = vec![0, 1, 1];
    
    let mut second_last_num = 1;
    let mut last_num = 1;
    for _i in 0..end-3 {
        let sum = second_last_num + last_num;
        result.push(sum);
        second_last_num = last_num;
        last_num = sum;
    }
    result
}