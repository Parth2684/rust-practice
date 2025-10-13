fn main() {
    let vec = vec!["Parth".to_string(), "Bhosle".to_string()];
    let vec_iter = vec.iter();
    // immutable iterator
    for val in vec_iter {
        println!("{}", val);
    }

    let mut vec1 = vec![1, 2, 3, 4];
    // mutable iterator
    let vec1_mut_iter = vec1.iter_mut();
    for val in vec1_mut_iter {
        *val += 1;
    } 
    println!("{:?}", vec1);

    // iterator using next
    let nums = vec![1,2,3];
    let mut iter = nums.iter();
    while let Some(val) = iter.next() {
        println!("{}", val);
    }

    let mut nums1 = vec![1,2,3,4];
    let mut nums1_mut_iter = nums1.iter_mut();
    while let Some(val) = nums1_mut_iter.next() {
        *val+=1;
    }
    println!("{:?}", nums1);

    // into iterator
    let into = vec![1,2,3];
    let mut into_iter = into.into_iter();
    while let Some(val) = into_iter.next() {
        println!("{}", val);
    }
    // takes ownership and cannot access into variable again

    // consuming adapters 
    let _sum: i32= into_iter.sum(); 
    // can't use into_iter variable again it tales the ownership use .clone() to not transfer the ownership
    
    // iterator adabpters 
    let vec = vec![1,2,3,4];
    let iter = vec.iter();
    let iter_returned = iter.map(|x| *x + 1);
    let iter_returned2 = iter_returned.clone().filter(|x| *x % 2 == 0);
    for i in iter_returned {
        println!("iterator adapter: {}", i);
    }

    for i in iter_returned2 {
        println!("filteres evens {}", i);
    }
}
 