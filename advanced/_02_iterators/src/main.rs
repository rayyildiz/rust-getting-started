use banner::print_banner;
use rand::prelude::*;

fn main() {
    print_banner();

    let infinite_loops = Numbers;

    for v in infinite_loops.take(5) {
        print!("{v:?} ");
    }

    println!("\nnow take usage:");

    let infinite_loop = Take {
        remaining_count: 10,
        inner: Numbers,
    };

    for v in infinite_loop {
        print!("{v:?} ");
    }
}

struct Numbers;

impl Iterator for Numbers {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let r: u8 = random();
        Some(r)
    }
}

struct Take<I> {
    inner: I,
    remaining_count: usize,
}

impl<I: Iterator> Iterator for Take<I> {
    type Item = <I as Iterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self.remaining_count {
            0 => None,
            _ => {
                self.remaining_count -= 1;
                self.inner.next()
            }
        }
    }
}
