mod math;

use math::math::ops;
use rust_intro::banner::print_banner;

fn main() {
    print_banner();

    crate::math::math::ops::print_version();

    println!("operations --- direct");
    println!("mul : {}", ops::multiple(3, 4));
    println!("add : {}", ops::add(3.4, 3.2));
    println!("div : {}", ops::divide(10, 4));
    println!("div2: {}", ops::divide(10.0, 4.0));

    println!("operations --- command");
    println!("ops add {}",ops::command(ops::Operation::Add,3,4));
    println!("ops div {}",ops::command(ops::Operation::Divide,39.2,4.1));
    println!("ops mul {}",ops::command(ops::Operation::Multiple,3.1,4.2));
}
