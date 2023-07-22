fn main() {
    let p = Person {
        first_name: "Ali".to_string(),
        last_name: "B".to_string(),
        country: "TR".to_string(),
        salary: 50_000,
    };

    let s = Student {
        first_name: "X".to_string(),
        last_name: "Y".to_string(),
        country: "US".to_string(),
        grade: "A".to_string(),
    };

    println!("person: {}", p.info());
    println!("student: {}", s.info());

    let c = Circle { rad: 3.2 };

    let r = Rectangle {
        height: 32.0,
        width: 32.0,
    };

    println!("circle {}", c.area());
}

struct Person {
    first_name: String,
    last_name: String,
    country: String,
    salary: i64,
}

struct Student {
    first_name: String,
    last_name: String,
    country: String,
    grade: String,
}

trait GeneralInformer {
    fn info(&self) -> &str;

    fn country(&self) -> &str;
}

impl GeneralInformer for Person {
    fn info(&self) -> &str {
        &self.first_name
    }

    fn country(&self) -> &str {
        &self.country
    }
}

impl GeneralInformer for Student {
    fn info(&self) -> &str {
        &self.grade
    }

    fn country(&self) -> &str {
        &self.country
    }
}

struct Circle {
    rad: f32,
}

struct Rectangle {
    width: f32,
    height: f32,
}

trait Calculator {
    fn area(&self) -> f32 {
        0.0
    }
}

impl Calculator for Circle {}

impl Calculator for Rectangle {}
