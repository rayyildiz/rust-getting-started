fn main() {
    let (_first, second) = (3, 4.3);
    let large_number = 1_000_000;

    println!("number {number:x}", number = large_number);

    let n1 = 14;
    let n2 = 12.2;

    let n3 = n1 as f64 + n2;

    println!("{}", n3);

    let mut name: &str = "Ramazan AYYILDIZ";
    println!("hello world, {}", name);
    name = "12345678901234567891234567890";
    println!("hello world, {}", name);

    let mut name = String::from("hello world");
    name.push_str(" ");
    println!(
        "
        string ops,
is_empty: {},
length: {},
cap: {},
contains: {}
        ",
        name.is_empty(),
        name.len(),
        name.capacity(),
        name.contains("world")
    );

    let n1 = "Ramazan".to_string();
    let n2 = "AYYILDIZ".to_string();
    let n3 = format!("{} {}", n1, n2);
    println!("full name: {}", n3);
}
