use rust_intro;

fn main() {
    rust_intro::print_banner();

    //let ten_millis = time::Duration::from_millis(100);
    // loop {
    //     print!(". ");
    //     sleep(ten_millis)
    // }

    let mut exit = true;
    let guess = 32;

    println!("guess a number");
    while exit {
        let mut number = String::new();

        std::io::stdin()
            .read_line(&mut number)
            .expect("failed to get number");

        let n: i32 = number.trim().parse().expect("failed to parse");
        if n == guess {
            exit = false;
            println!("congrats, program will exit");
        } else {
            println!("wrong, enter again");
        }
    }
}
