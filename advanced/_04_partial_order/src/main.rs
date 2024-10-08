use banner::print_banner;
use std::cmp::Ordering;

fn main() {
    print_banner();

    let p = Person {
        age: 34,
        name: "Ramazan".to_string(),
    };

    let p2 = Person {
        age: 17,
        name: "Ramazan".to_string(),
    };

    println!("equals : {}", p == p2);
    println!("greater than : {}", p > p2);
    println!("less than : {}", p < p2);

    let input = Some(3);
    if let Some(num) = input {
        println!("{num} value has to be 3");
    }
}

fn main2() {
    let number = String::from("32");
    println!("main : {number}");
    foo(number);
    //println!("main: {number}");
}

fn foo(a: String) {
    println!("foo: {a}");
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u16,
}

impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let option = if self.age > other.age {
            Ordering::Greater
        } else if self.age == other.age {
            Ordering::Equal
        } else {
            Ordering::Less
        };
        Some(option)
    }
}
