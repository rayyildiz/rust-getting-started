use rust_intro::banner::print_banner;

fn main() {
    print_banner();

    let s1 = "hello";
    let v: &str;

    {
        let s2 = String::from("jaja");
        v = which(s1, s2.as_str());
    }

    println!("v : {}", v);

    let a = 10;
    let result;
    {
        let b = 5;
        result = greater_life(&a, &b);
        println!("bigger {}", result);
    }

    let first_name = "Ramazan";
    let mut p = Person {
        age: 2,
        name: &first_name,
    };

    {
        let last_name = String::from("AYYILDIZ");
        p.name = &last_name;
        println!("person :{:?}", p);
    }

    let v = vec![1, 2, 3, 4];
    let b = max_vec(&v, &v);
    println!("max v: {:?}", b);
}

fn which<'a>(s: &'a str, b: &str) -> &'a str {
    s
}

// no need lifetime operator
fn greater(a: &i32, b: &i32) -> i32 {
    *a
}

fn greater_life<'a, 'b>(a: &'a i32, b: &'a i32) -> &'a i32 {
    if a > b {
        a
    } else {
        b
    }
}

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: i32,
}

fn max_vec<'a>(a: &'a [i32], b: &'a [i32]) -> &'a [i32] {
    a
}
