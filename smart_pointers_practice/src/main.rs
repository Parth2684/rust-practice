use std::rc::Rc;

fn main() {
    let var = 5;
    let var1 = Box::new(1);
   println!("{}", var * *var1); 
   
   let str = String::from("Hi there");
   {
       let str_rc = Rc::new(str);
       println!("{:?}", Rc::strong_count(&str_rc));
       {
           let str_rc2 = Rc::clone(&str_rc);           
           println!("{}", Rc::strong_count(&str_rc2))
       }
   }
}
