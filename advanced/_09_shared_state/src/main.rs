use banner::print_banner;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    print_banner();

    let mx = Mutex::new(5);

    {
        let mut num = mx.lock().unwrap();
        *num = 10;

        thread::sleep(Duration::from_secs(1));
    }

    println!("{mx:?} ");

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = counter.clone();
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("lock {}", counter.lock().unwrap());
}
