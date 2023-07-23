use rust_intro::banner::print_banner;

fn main() {
    print_banner();

    let mut v = vec![1, 42, 122, 41, 33, 122, 3];

    for i in 0..v.len() {
        println!("index {}, val:{}", i, v[i]);
    }

    println!("-------");
    for i in v.iter_mut() {
        if *i == 42 {
            continue;
        }
        *i *= 5;
        println!("val: {}", i);

        if *i == 205 {
            break;
        }
    }

    println!("changed version: {:?}", v);

    let mut x = 100;
    loop {
        x = x - 1;
        if x % 13 == 0 {
            break;
        }
    }

    println!("found 13 divied number {}", x);

    let mut count = 0;
    let xx = loop {
        count += 1;

        if count % 5 == 0 && count % 9 == 0 {
            break count * 5;
        }
    };

    println!("some loop let, count: {}, xx: {}", count, xx);
}
