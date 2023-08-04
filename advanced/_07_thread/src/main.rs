use banner::print_banner;

use std::thread;

fn main() {
    print_banner();

    let mut thread_vec = vec![];
    for i in 0..10 {
        thread_vec.push(thread::spawn(move || {
            println!("thread_{i}");
        }));
    }

    for t in thread_vec {
        t.join();
    }

    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("inside thread: {v:?}");
    });

    // cant compile, v is moved to thread
    // drop(v);

    handle.join();
}
