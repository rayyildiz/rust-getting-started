use banner::print_banner;
use std::sync::mpsc;
use std::thread;
use std::time::Instant;

fn main() {
    print_banner();
    let start = Instant::now();
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        for i in 1..10_000_001 {
            tx.send(i as u64).unwrap();
        }
    });

    let mut sum = 0;
    for a in rx {
        sum += a;
    }
    let diff = Instant::elapsed(&start);
    println!("sum :{}. elapsed: {:?}", sum, diff);
}
