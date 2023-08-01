use banner::print_banner;

fn main() {
    print_banner();

    let name = Some(String::from("Alice"));
    let name_ref = name.as_ref();

    match name_ref {
        Some(ref_str) => {
            println!("Name: {}", ref_str);
        }
        None => {
            println!("No name available");
        }
    }
}
