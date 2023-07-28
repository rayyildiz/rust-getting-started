use banner::print_banner;
macro_rules! input {
    ($t:ty) => {{
        let mut m = String::new();
        std::io::stdin()
            .read_line(&mut m)
            .expect("failed to read stdin");

        let a: $t = m.trim().parse().expect("failed to parse");
        a
    }};
}

macro_rules! create_function {
    ($func_name:ident, $input:ident, $type_input:ty, $type_output:ty) => {
        fn $func_name($input: $type_input) -> $type_output {
            println!(
                "fn invoked {:?} ( {:?}:{:?})",
                stringify!($func_name),
                stringify!($input),
                stringify!($type_input)
            );
            $input
        }
    };
}

macro_rules! str_macro {
    ($($s:expr), * $(,)?) => {{
        let mut t = String::new();
        $(t.push_str($s);)*

        t
    }}
}

create_function!(f1, xy, i32, i32);

fn main() {
    print_banner();

    let s = str_macro!("a","b","c",);
    println!("concat str1: {}",s);
    println!("concat str2: {}",str_macro![]);
    println!("concat str3: {}",str_macro!["a"]);
    println!("concat str4: {}",str_macro!["a","b",]);
    println!("concat str5: {}",str_macro!["a","b"]);



    println!("please enter a valid integer");
    let a = input!(f32);
    println!("entered value {}", a);

    let b = f1(3);
    println!("result {}", b);
}
