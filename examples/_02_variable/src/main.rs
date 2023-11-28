use banner::print_banner;

fn main() {
    print_banner();

    let mut x: f32 = 32.0;

    println!(" x value {}", x);

    x = 34.0;

    println!(" x value {}", x);

    println!("i8  min: {}, max  {}", i8::MIN, i8::MAX);
    println!("i16 min: {}, max  {}", i16::MIN, i16::MAX);
    println!("i32 min: {}, max  {}", i32::MIN, i32::MAX);
    println!("i64 min: {}, max  {}", i64::MIN, i64::MAX);
    println!("u8  min: {}, max  {}", u8::MIN, u8::MAX);
    println!("u16 min: {}, max  {}", u16::MIN, u16::MAX);
    println!("u32 min: {}, max  {}", u32::MIN, u32::MAX);
    println!("u64 min: {}, max  {}", u64::MIN, u64::MAX);

    println!("f32 min: {}, max  {}", f32::MIN, f32::MAX);
    println!("f64 min: {}, max  {}", f64::MIN, f64::MAX);

    println!("status {:?}", (true, 32.4, 344));

    let mut x1 = 40;
    let x2;
    x1 = x1 * 3;
    x2 = x1 - 2;
    println!("My age is {} and my son age is {}", x1, x2);

    let name = String::from("Ramazan");
    println!("name : {}", name);
    foo(name);
    // println!("name : {}",name);
}

fn foo(s: String) {
    println!("hello  {s}");
}
