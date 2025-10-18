use std::{sync::mpsc, thread};

fn main() {
    let x = 1;
    let (tx, rx) = mpsc::channel();
 
    {
        let v = vec![1,2,3];
        thread::spawn(move || {
            println!("{v:?}");
            tx.send(v).unwrap();
        });
    }
    let receiver = rx.recv().unwrap();
    println!("{receiver:?}");
    
    println!("{x}")
}
