#[derive(Debug)]
struct ResultIsEven {
    number: u16,
    is_even: IsEvenType
}
#[derive(Debug)]
enum IsEvenType {
    Even,
    Odd
}


fn main() {
    for item in is_even() {
        println!("{} is {:?}", item.number, item.is_even)
    }
}


fn is_even() -> Vec<ResultIsEven> {
    let mut result:Vec<ResultIsEven> = vec![];
    for i in 0u16..=65535 {
        if i%2 == 0 {
            result.push(ResultIsEven {
                number: i,
                is_even: IsEvenType::Even
            });
        }else {
            result.push(ResultIsEven {
                number: i,
                is_even: IsEvenType::Odd
            });
        }
    }
    result
}
