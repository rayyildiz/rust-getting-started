fn main() {
    let mut disase: Option<String> = None;

    disase = Some("test".to_string());
    disase = None;

    match disase {
        Some(s) => println!("match val: {}", s),
        None => println!("None"),
    }

    let s = Some("test");
    println!("unwrap {}, or {:?}", s.unwrap(), s);

    println!("func some {:?}", square(Some(3)));
    println!("func none {:?}", square(None));
}

fn square(s: Option<i32>) -> Option<i32> {
    match s {
        Some(s) => Some(s * s),
        None => None,
    }
}
