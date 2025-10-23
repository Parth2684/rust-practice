use std::{sync::mpsc, thread};

fn main() {
    for i in 0..=10 {
        let handle = thread::spawn(move || println!("hello world from {} thread", i));
        handle.join().unwrap();
    }

    let (trs, rec) = mpsc::channel();
    let val = String::from("transmitter");

    thread::spawn(move || trs.send(val).unwrap());

    let receive = rec.recv().unwrap();
    
    
    println!("{:?}", receive);
    println!("Hello from outside the thread");
}
