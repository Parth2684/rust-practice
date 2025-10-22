use std::rc::Rc;

use std::cell::RefCell; 


struct Flag{
    is_true: Rc<RefCell<bool>>
}
fn main() {
   let t = (1, "hi there");
   let b = Box::new(t);
   println!("{b:?}");
   
   let x = 5;
   let y = &x;
   
   assert_eq!(x, 5);
   assert_eq!(*y, 5);
   
   let p = 5;
   let q = Box::new(p);
   assert_eq!(p, 5);
   assert_eq!(*q, 5);
   
   let a = Rc::new(String::from("Hi there"));
   let b = Rc::clone(&a);
   println!("{:?} {b:?}",a.contains("Point"));
   
   let flag = Flag{
       is_true: Rc::new(RefCell::new(true)),
   };
   
    let immutborrow = Rc::new(flag.is_true.clone());
    println!("{immutborrow:?}");
    
    let mut mutborrow = immutborrow.borrow_mut();
    *mutborrow = false;
    
    println!("{:?}", mutborrow);
}