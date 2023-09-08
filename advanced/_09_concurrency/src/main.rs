use banner::print_banner;
use std::thread;
use std::time::Duration;
use threadpool::ThreadPool;

fn main() {
    print_banner();

    static ALPHABET: [char; 26] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];

    let mut my_list = vec![];
    for i in 0..100 {
        let index = i % 26;
        let key = format!("{}{}", i, ALPHABET[index]);
        let value = format!("{}{}", ALPHABET[index], ALPHABET[index]);
        my_list.push(Person {
            name: key,
            surname: value,
        })
    }


    let pool = ThreadPool::new(20);

    for mut person in my_list {
        pool.execute(move || {
            println!("updating {person:?}, {:?}", thread::current().id());
            person.surname = format!("{}...", person.surname);
            thread::sleep(Duration::from_secs(1));
            println!("updated {person:?}, {:?}", thread::current().id());
        });
    }

    pool.join();
}

#[derive(Debug)]
struct Person {
    name: String,
    surname: String,
}
