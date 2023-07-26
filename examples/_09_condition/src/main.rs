use banner::print_banner;

fn main() {
    print_banner();

    let a = 34;

    if a > 24 && a < 50 {
        println!("between")
    } else if a == 35 {
        println!("izmir");
    } else {
        println!("not between");
    }

    let result = if a > 24 && a < 50 {
        'A'
    } else if a == 35 {
        'B'
    } else {
        'C'
    };

    println!("result \'{}\'", result);

    match result {
        'A' => println!("A success"),
        'B' | 'C' => println!("well"),
        _ => println!("not equal"),
    };

    let grade = 50;

    let grade_str = match grade {
        85..=100 => 'A',
        70..=84 => 'B',
        55..=69 => 'C',
        45..=54 => 'D',
        0..=44 => {
            let b = 22;
            'E'
        }
        _ => 'F',
    };

    println!("Grade {}", grade_str);
}
