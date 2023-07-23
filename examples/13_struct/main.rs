use rust_intro::banner::print_banner;

fn main() {
    print_banner();

    let ramazan = Person {
        first_name: String::from("Ramazan"),
        last_name: String::from("AYYILDIZ"),
        country: String::from("TR"),
        age: 41,
        salary: 50_000,
    };

    println!("ramazan1: {:?}", Person::calculate_tax(&ramazan));
    println!("ramazan2: {:?}", ramazan.calculate_tax());

    let p1 = Person::new();

    let mut p2 = Person {
        first_name: "ABC".to_string(),
        last_name: "ABBX".to_string(),
        ..p1
    };

    p2.country = "US".to_string();
    println!("p2: {:?}", p2.country);

    let n1 = Complex(1, 3.0);
    let n2 = Complex(2, 4.0);

    println!("greater {}", n1.greater(&n2));
}

struct Person {
    age: i32,
    first_name: String,
    last_name: String,
    country: String,
    salary: i64,
}

impl Person {
    fn new() -> Person /* Self */ {
        // Person {
        Self {
            age: 0,
            first_name: "".to_string(),
            last_name: "".to_string(),
            country: "".to_string(),
            salary: 0,
        }
    }

    fn calculate_tax(&self) -> f32 {
        (self.salary as f32) * 0.15
    }
}

struct Complex(i32, f32);

impl Complex {
    fn greater(&self, another: &Complex) -> bool {
        let a = self.0 > another.0;
        let b = self.1 > another.1;

        a && b
    }
}
