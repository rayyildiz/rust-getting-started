use banner::print_banner;

use std::sync::mpsc;
use std::thread;

fn main() {
    print_banner();

    let (sender, receiver) = mpsc::channel();

    thread::spawn(move || {
        let v = "hello channel".to_string();
        println!("sender: sending a data");
        sender.send(v).unwrap();
        println!("sender: sent a message")
    });

    for r in receiver.iter() {
        println!("receiver: got {r}");
    }
}
