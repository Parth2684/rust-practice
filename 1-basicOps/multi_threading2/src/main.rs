use std::{sync::mpsc, thread, time::Instant};
fn main() {
    let (tx, rx) = mpsc::channel();
    let start = Instant::now();
    for i in 0..10 {
        let producer = tx.clone();
        thread::spawn(move || {
            let mut ans: u128 = 0;
            for j in 0..=100_000_000 {
                ans = ans + (i*100_000_000 + j);
            }
            producer.send(ans).unwrap();
        });
    }
    drop(tx);
    let mut ans = 0;   
    for val in rx {
        ans += val;
    }
    println!("{ans}");
    let elapsed = start.elapsed();
    println!("{:?}", elapsed);
}
