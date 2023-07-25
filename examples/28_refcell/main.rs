use std::cell::RefCell;
use std::rc::Rc;



fn main() {
    let a  = RefCell::new(3);
    let mut b = a.borrow_mut();
    *b = 30;
    drop(b);

    println!("a vale {:?}",a);

    let java = Rc::new(RefCell::new("java".to_string()));
    *java.borrow_mut() = "Rust".to_string();
    println!("java is now {:?}",java);
}