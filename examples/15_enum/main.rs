use rust_intro;

fn main() {
    rust_intro::print_banner();

    let participant = Conveyance::Air;

    println!("car :{}", Conveyance::Car as i32);
    println!("train :{}", Conveyance::Train as i32);
    println!("air :{}", Conveyance::Air as i32);

    println!("travel {}", participant.travel(39));

    let vals = vec![Value::Integer(34), Value::Float(2.3)];
    println!("values {:?}", vals);

    for i in vals.iter() {
        match i {
            Value::Integer(num) => println!("i32 : {}", num),
            Value::Float(num) => println!("f32: {}", num),
        }
    }
}

enum Conveyance {
    Car = 1 << 0,
    Train = 1 << 1,
    Air = 1 << 2,
}

impl Conveyance {
    fn travel(&self, km: i32) -> f32 {
        let re = match self {
            Conveyance::Car => 1.1 * 1.0,
            Conveyance::Train => 1.2 * 2.0,
            Conveyance::Air => 1.3 * 3.1,
        };

        re * 1.2 * km as f32
    }
}

#[derive(Debug)]
enum Value {
    Integer(i32),
    Float(f32),
}
