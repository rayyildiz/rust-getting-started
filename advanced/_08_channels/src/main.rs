use banner::print_banner;

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    print_banner();

    let (sender, receiver) = mpsc::channel();
    let sender_clone = sender.clone();
    thread::spawn(move || {
        let v = vec![1, 2, 3, 4, 5];
        println!("sender1: sending a data");
        for i in v {
            sender.send(i).unwrap();
            thread::sleep(Duration::from_millis(400));
        }
        println!("sender1: sent all messages")
    });

    thread::spawn(move || {
        let v = vec![11, 12, 13, 14, 15];
        println!("sender2: sending a data");
        for i in v {
            sender_clone.send(i).unwrap();
            thread::sleep(Duration::from_millis(800));
        }
        println!("sender2: sent all messages")
    });

    for r in receiver.iter() {
        println!("receiver: got {r}");
    }

    println!("send all message in a different thread");
    let (tx, rx) = mpsc::channel();
    for i in 0..5 {
        let a = tx.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_secs(1));
            a.send(i).unwrap();
        });
    }
    drop(tx);

    for r in rx.iter() {
        println!("receiver: got {r}");
    }
}
