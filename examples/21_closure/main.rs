use rust_intro;

fn main() {
    rust_intro::print_banner();

    let x = 5;

    let square = |num: i32| println!("square: {}", num * num);
    square(x);

    let square = |num: i32| num * num * num;
    let qube = square(25);
    println!("qube or square {}", qube);

    let sum = |a, b| a + b;
    let sum1 = sum(3, 4);
    println!("sum1 {:?}", sum1);

    // because of first call it is integer, cant be float
    // let sum2 = sum(2.1, 2.3);

    let can_divide = |a: f32| {
        if a == 0.0 {
            false
        } else {
            true
        }
    };

    let d = divide(4.0, 2.3, can_divide);
    println!("d :{}", d);

    let mut v = vec![1, 2, 2, 2];
    let mut add = || {
        v.push(23);
    };

    add();
    println!("vec {:?}", v);

    let v1 = vec![12, 3];
    let ff = || {
        let v2 = v1;
    };

    ff();

    // compile error. because ownership transferred to v2 in the closure
    //println!("v1 :{:?}",v1);
}

fn divide<F: Fn(f32) -> bool>(a: f32, b: f32, f: F) -> f32 {
    if f(b) {
        a / b
    } else {
        0.0
    }
}
