use rust_intro;
use std::env::var;

fn main() {
    rust_intro::print_banner();

    let s1 = String::from("hello");
    {
        let s2 = &s1;
        println!("s1: {}, s2:{}", s1, s2);
    }
    println!("s1: {}", s1);

    let mut v1 = vec![1, 2];
    let v2 = v1.clone();
    v1.push(2);
    println!("v2 {:?} ", v2);

    let stack_num = 32;
    let mut heap_vec = vec![4, 5, 6];

    stack_fn(stack_num);
    println!("outside {}", stack_num);

    heap_fn(&mut heap_vec);
    println!("after fn {:?}", heap_vec);

    let s1 = String::from("hello");
    let s2 = String::from("world");

    let merge_vec = vec![&s1, &s2];

    println!("merge_vec  {:?}", merge_vec);
}

fn stack_fn(mut stack_num: i32) {
    stack_num = 56;
    println!("inside func {}", stack_num);
}

fn heap_fn(var: &mut Vec<i32>) {
    var.push(3);

    println!("inside fn {:?}", var);
}
