fn main() {
    let m = Box::new(1.2);
    let a = 1.2;

    println!("equals {}", a == *m);

    let s_1 = 3.4;
    let s_2 = &s_1;

    let h_1 = Box::new(s_1);
    let h_2 = Box::new(s_2);

    println!("h1: {}, h2: {}, s1:{}, s2:{}", h_1, h_2, s_1, s_2);

    let l = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));

    println!("list {:?}", l);

    let x = 50;
    let b = &x;
    println!("equals x ? {}", x == 50);
    println!("equals b ? {}", *b == 50);
    //println!("equals b == x {}", x == b);

    let l1 = LinkedList::new(x);
    let l2 = LinkedList::new(*b);

    let l3 = LinkedList::new(x + 1);
    let l4 = LinkedList::new(x + 4);
    let l5 = LinkedList::new(x + 6);

    println!("deref {}", *l1 == x);
    println!("deref {}", x == *(l1.deref()));

    drop(l4);
}

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct LinkedList {
    value: i32,
}

impl LinkedList {
    fn new(v: i32) -> Self {
        LinkedList { value: v }
    }
}

use std::ops::Deref;
use std::ops::Drop;

impl Deref for LinkedList {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Drop for LinkedList {
    fn drop(&mut self) {
        println!("dropping value {}", self.value);
        self.value = 0;
    }
}
