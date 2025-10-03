fn main () {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    even_out(&mut vec);
    println!("{:?}", vec);
}

fn even_out (vec: &mut Vec<i32>) {
    let mut i = 0;
    while i < vec.len() {
        if vec[i] % 2 != 0 {
            vec.remove(i);
        }else {
            i+=1;
        }
    }
}