use banner::print_banner;

fn main() {
    print_banner();

    let f = Fibonacci(0, 1);

    println!("f: {f:?}");

    for x in f.take(10) {
        println!("fib: {x:?}");
    }
}

#[derive(Debug)]
struct Fibonacci<T>(T, T);

impl<T> Iterator for Fibonacci<T>
where
    T: std::ops::Add<Output = T> + Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let sum = self.0 + self.1;

        self.0 = self.1;
        self.1 = sum;

        Some(self.0)
    }
}
