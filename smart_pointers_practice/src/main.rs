use std::{rc::Rc, sync::{mpsc, Arc, Mutex}, thread::{self, JoinHandle}};

fn main() {
    let var = 5;
    let var1 = Box::new(1);
   println!("{}", var * *var1); 
   
   let str = Rc::new(String::from("Hi there"));
   {
       let str_rc = Rc::clone(&str);
       println!("{:?}", Rc::strong_count(&str_rc));
       {
           let str_rc2 = Rc::clone(&str_rc);           
           println!("{}", Rc::strong_count(&str_rc2))
       }
       println!("{}", Rc::strong_count(&str_rc));
   }
   
   let arc = Arc::new(String::from("Hi there"));
   let arc1 = Arc::clone(&arc);
   
   let (transmit, receive) = mpsc::channel();
   
   thread::spawn(move || {
       println!("{}", arc);
       transmit.send(arc).unwrap();
       println!("{}", arc1);
       transmit.send(arc1).unwrap();
   });
   
   let recieved = receive.recv().unwrap();
   println!("{:?}", recieved);
   
   
   let counter = Arc::new(Mutex::new(0));
   for i in 0..11 {
       let counter = Arc::clone(&counter);
       let handle = thread::spawn(move || {
           let mut num = counter.lock().unwrap();
           // let mut _num2 = counter.lock().unwrap(); // it creates a deadlock
           *num +=1;
           println!("{} at {} thread",*num, i);
       });
       handle.join().unwrap()
   }
}
