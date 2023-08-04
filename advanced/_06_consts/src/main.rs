use banner::print_banner;
use std::ops::Deref;

fn main() {
    print_banner();

    let mut a = vec![5, 3, 6, 8, 2, 3];
    a.sort();
    let a = a;
    println!("vector {a:?}");

    let b = {
        let mut b = vec![1, 2, 5, 3, 99, 5];
        b.sort();
        b
    };
    println!("vector {b:?}");

    let mut c = Configuration::new();
    c.name = "my app".to_string();
    let f = c.build();

    let mut f_2 = f;
    // it onwt compile, because it has only deref for immutable object,
    // it doesnt have a mut deref
    //f_2.port =  3000;

    println!("f2 {f_2:?}");
}

#[derive(Debug, Clone)]
pub struct final_config<T>(T);

impl<T> Deref for final_config<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Copy for final_config<T> where T: Copy {}

#[derive(Debug)]
struct Configuration {
    port: u16,
    name: String,
}

impl Configuration {
    fn new() -> Self {
        Configuration {
            port: 4000,
            name: "app".to_string(),
        }
    }

    fn build(self) -> final_config<Configuration> {
        final_config(self)
    }
}
