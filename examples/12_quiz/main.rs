use rust_intro;

fn main() {
    rust_intro::print_banner();

    let x: u64 = 4_294_967_296;
    let y = x as u32;
    if x == y as u64 {
        println!("x equals y.");
    } else {
        println!("x does not equal y.");
    }

    let s1 = String::from("Hello");
    let s2 = &s1;
    let s3 = &s2;
    let s4 = &s3;

    println!("{}", s4);

    let a = ***s4 == "Hello".to_string();
    println!("aa: {}", a);
}
