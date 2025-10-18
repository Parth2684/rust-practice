use std::thread;

fn main() {
    thread::spawn(|| {
        for i in 0..5000000 {
            println!("from spawned thread one {}", i);
        }
    });
    
    thread::spawn(|| {
        for i in 0..5000000 {
            println!("from spawned thread two {}", i);
        }
    });
    
    thread::spawn(|| {
        for i in 0..5000000 {
            println!("from spawned thread three {}", i);
        }
    });
    
    
    for i in 0..5000000 {
        println!("from main thread {i}");
    }
}
