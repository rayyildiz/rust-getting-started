use rust_intro::banner::print_banner;

fn main() {
    print_banner();

    let f = max;
    println!("f :{}", f(2, 3));

    print_info(max, 3, 4);
    print_info(min, 3, 4);

    let sum = |a: i32, b: i32| a + b;
    print_info(sum, 3, 4);
}

fn print_info(f: fn(i32, i32) -> i32, a: i32, b: i32) {
    println!("hello from info . calling fn");
    let c = f(a, b);
    println!("result {}", c);
}

/// max result
/// # Tests
/// ```
/// let m = max(20,4);
///
/// assert_eq!(20,m);
/// ```
fn max(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

fn min(a: i32, b: i32) -> i32 {
    if a < b {
        a
    } else {
        b
    }
}
