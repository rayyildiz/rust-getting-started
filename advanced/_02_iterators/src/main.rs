use banner::print_banner;

fn main() {
    print_banner();

    let infinite_loops = Ones;

    for one in infinite_loops.take(5) {
        print!("{one:?} ");
    }

    println!("\nnow take usage:");

    let infinite_loop = Take {
        remaining_count: 10,
        inner: Ones,
    };

    for v in infinite_loop {
        print!("{v:?} ");
    }
}

struct Ones;

impl Iterator for Ones {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(1)
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
