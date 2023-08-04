use _05_struct_init::{Class, Person};
use banner::print_banner;

fn main() {
    print_banner();

    //compile error
    /*
    let p = Person{
        name: "Ramazan".to_string(),
        birth_year: 2000,
        age: 0,
    };
    */

    let p = Person::new("xsaxsa".to_string(), 1988);
    println!("init {p:?}");

    let p2 = Person::new("ab".to_string(), 1960).unwrap_or_default();

    println!("init or default {p2:?}");

    let c = Class {
        capacity: 50,
        ..Default::default()
    };
    println!("class {c:?}");
}
