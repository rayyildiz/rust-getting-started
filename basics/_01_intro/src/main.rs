use banner::print_banner;

fn main() {
    print_banner();
    println!("Hello, world!");

    dbg!(
        "hello world
sxsa
xsa
xsa
      "
    );

    println!("your age is {}", 10);

    println!(
        "hello {} - {},\n\t how are
    you?",
        "Ramazan", "AYYILDIZ"
    );

    println!("hello {name}", name = "Ramazan");
}
