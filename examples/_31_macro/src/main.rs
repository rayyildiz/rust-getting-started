macro_rules! ayyildiz {
    () => {
        1 + 1;
    };
    (star) => {
        1 + 3
    };
    ($p1:expr, $p2:expr, $p3:expr) => {
        $p3 * ($p1 + $p2)
    };
}

fn main() {
    println!("Hello, world!");

    ayyildiz!();

    println!("macro result: {}", ayyildiz!());
    println!("macro result, second rule: {}", ayyildiz!(star));
    println!("multiple {}", ayyildiz![4, 5, 3]);
    println!("multiple {}", ayyildiz!(1.2, 5.3, 3.4));

    ayyildiz![star];

}
