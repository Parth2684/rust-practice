use std::collections::{BinaryHeap};


fn main() {
    // let mut nums: Vec<i32> = Vec::new();
    // nums.push(1);
    // nums.push(2);
    // nums.push(3);
    // let pop = nums.pop();
    // match pop {
    //     Option::Some(num) => println!("Number popped was {num}"),
    //     Option::None => println!("No element in the vector")
    // }
    // let last_index = nums.len();
    // println!("{last_index}");
    // nums.insert(last_index, 10);
    // println!("{nums:?}");
    // nums.remove(1);
    // println!("{nums:?}");
    
    
    let mut bheap = BinaryHeap::new();
   
   bheap.push(1);
   bheap.push(18) ;
   bheap.push(20);
   bheap.push(5);
   println!("{bheap:?}");
}   
