use rust_intro::banner::print_banner;

fn main() {
    print_banner();

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
        println!("you entered: {}",n);
        if n == guess {
            exit = false;
            println!("congrats, {} is correct answer.  program will exit",n);
        } else {
            println!("wrong, enter again");
        }
    }
}
