use rust_intro::banner::print_banner;

fn main() {
    print_banner();

    let info = ("Ramazan", "AYYILDIZ", 41);

    println!("name :{}", info.0);

    let complex = ("ENG", (true, 42, (12, 22, (true, 3))));
    let a = complex.1 .2 .2 .1;
    println!("{}", a);

    let i = 3;

    let arr = [1, 2, 3, 4];
    println!("{}", arr[i]);

    let mut number_array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("number array {:?}", number_array);

    let subset_array = &number_array[0..=3];
    // number_array[0] = 1;
    println!("subset array {:?}", subset_array);

    println!(
        "number_array size in mem: {}",
        std::mem::size_of_val(&number_array)
    );

    println!(
        "subset_array size in mem: {}",
        std::mem::size_of_val(subset_array)
    );
}
