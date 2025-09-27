#[derive(Debug)]
struct ResultIsEven {
    number: u16,
    is_even: bool
}


fn main() {
    println!("{:?}", is_even())
}


fn is_even() -> Vec<ResultIsEven> {
    let mut result:Vec<ResultIsEven> = vec![];
    for i in 0u16..=65535 {
        if i%2 == 0 {
            result.push(ResultIsEven {
                number: i,
                is_even: true
            });
        }else {
            result.push(ResultIsEven {
                number: i,
                is_even: false
            });
        }
    }
    result
}
