use banner::print_banner;

fn main() {
    print_banner();

    let opt = Some("hello".to_owned());
    match opt.as_ref() {
        // or match &opt {
        None => println!("none value"),
        // Some(ref s) => println!("some value {}",s), // not idiomatic. use &opt
        Some(s) => println!("some value {}", s),
    }

    println!("opt val: {:?}", opt);

    let op = Some("rust".to_owned());
    let a = &op;
    let b = op.as_ref();
}
