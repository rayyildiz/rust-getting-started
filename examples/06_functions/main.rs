use rust_intro;

fn main() {
    rust_intro::print_banner();

    let a = sub_total(2, 4);
    println!("result : {}", a);

    let b = return_tuple("hello world");
    println!("result {:?}", b);

    let (empty, count) = return_tuple("why");
    println!(
        "results : {empty}, count:{count}",
        empty = empty,
        count = count
    );

    let name = {
        let first_name = "Ramazan";
        let last_name = "AYYILDIZ";
        format!("{} {}", first_name, last_name)
    };

    println!("name: {}", name);
    print!("\nenter a value: ");

    let mut n = String::new();

    std::io::stdin().read_line(&mut n).expect("failed to read");

    let number: f32 = n.trim().parse().expect("failed to parse number");

    println!("entered value \"{}\". number: {}", n, number);
}

fn sub_total(x: i32, y: i32) -> i32 {
    2 * x + 3 * y
}

fn return_tuple(x: &str) -> (bool, u32) {
    let s = String::from(x);
    let is_empty = s.is_empty();
    let count = s.len();

    (is_empty, count as u32)
}
