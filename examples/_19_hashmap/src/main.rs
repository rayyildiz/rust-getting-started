use banner::print_banner;
use std::collections::HashMap;

fn main() {
    print_banner();

    let mut map: HashMap<&str, Person> = HashMap::new();

    map.insert(
        "k1",
        Person {
            name: "A".to_string(),
        },
    );
    map.insert(
        "v1",
        Person {
            name: "B".to_string(),
        },
    );
    map.insert(
        "c1",
        Person {
            name: "C".to_string(),
        },
    );

    println!("{:?}", map.get("v1").unwrap());

    if map.contains_key("v1") {
        println!("exists");
    }

    match map.get("v222") {
        Some(s) => println!("has value {:?}", s),
        None => println!("not exist"),
    };

    for (k, v) in &map {
        println!("key: {}, value: {:?}", k, v);
    }

    let mut m: HashMap<&str, &str> = HashMap::new();
    m.entry("k1").or_insert("kee");
    m.entry("k1").or_insert("lee");

    println!("values {:?}", m);

    let numbers = vec![1, 3, 1, 35, 5, 3, 2, 13, 3, 5, 12, 42, 1];
    let mut count_map: HashMap<i32, i32> = HashMap::new();

    for number in &numbers {
        let freq = count_map.entry(*number).or_insert(0);
        *freq += 1;
    }
    for (k, v) in &count_map {
        println!("number: {}, count: {:?}", k, v);
    }
}

#[derive(Debug)]
struct Person {
    name: String,
}
