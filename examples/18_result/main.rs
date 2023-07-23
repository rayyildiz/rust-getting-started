use rust_intro;

fn main() {
    rust_intro::print_banner();

    println!("ok {:?}", divide(4.3, 2.1));
    println!("err {:?}", divide(4.3, 0.0));

    let v = vec![1, 2, 3, 4, 5, 6, 4];
    let a = match v.get(6) {
        Some(c) => Ok(c),
        None => Err("no data".to_string()),
    };

    println!("result {:?}", a);
}

fn divide(d1: f64, d2: f64) -> Result<f64, String> {
    /*
    if d2 == 0.0 {
        Err("zero division".to_string())
    }else{
        Ok(d1 / d2)
    }
    */
    match d2 {
        0.0 => Err("zero division".to_string()),
        _ => Ok(d1 / d2),
    }
}
