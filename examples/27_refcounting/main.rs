use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    use List::{Cons, Nil};

    let a = Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(Nil)))));
    println!("1. count of a: {}", Rc::strong_count(&a));
    {
        let b = Rc::new(Cons(4, Rc::clone(&a)));
        println!("2. count of a: {}", Rc::strong_count(&a));
    }
    println!("3. count of a: {}", Rc::strong_count(&a));
    let c = Rc::new(Cons(5, Rc::clone(&a)));
    println!("4. count of a: {}", Rc::strong_count(&a));
}
