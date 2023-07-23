use rust_intro;

fn main() {
    rust_intro::print_banner();

    println!("integer {}", square(4));
    println!("float {}", square(2.4));

    let p = Point { x: 3.1, y: 4 };

    println!("int {:?}", Point { x: 3, y: 4 });
    println!("float {:?}", p);

    p.print()
}

fn square<T>(n: T) -> T
where
    T: std::ops::Mul<Output = T> + std::ops::Add<Output = T> + Copy,
{
    let a = n + n;
    n * n * a
}

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U>
where
    T: std::fmt::Display,
    U: std::fmt::Display,
{
    fn print(&self) {
        println!("x:{}, y:{}", self.x, self.y)
    }
}
