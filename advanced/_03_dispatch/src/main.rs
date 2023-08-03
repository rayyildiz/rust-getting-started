use banner::print_banner;

fn main() {
    print_banner();

    let a = Number;
    static_display(a);

    let b = 233_i32;
    static_display(b);

    static_display("abc".to_string());

    dynamic_dispatch(&Person {
        name: "aazza".to_string(),
    });
    dynamic_dispatch(&Customer {
        name: "aazza".to_string(),
    });

    dynamic_dispatch_vec(vec![
        &Person {
            name: "aazza".to_string(),
        },
        &Customer {
            name: "aazza".to_string(),
        },
    ]);
}

trait Print {
    fn print(&self);
}

struct Number;

impl Print for Number {
    fn print(&self) {
        println!("number object");
    }
}

impl Print for i32 {
    fn print(&self) {
        println!("i32: {}", self);
    }
}

impl Print for String {
    fn print(&self) {
        println!("string: {}", self);
    }
}

fn static_display<T: Print>(x: T) {
    x.print();
}

/*
rust generates:
this is static dispatch

fn static_display_i32(x:i32) {}

fn static_display_String(x:String) {}

fn static_display_Number(x:Number) {}

*/

#[derive(Debug)]
struct Person {
    name: String,
}

#[derive(Debug)]
struct Customer {
    name: String,
}

trait Info {
    fn info(&self);
}

impl Info for Person {
    fn info(&self) {
        println!("person _ {}", self.name);
    }
}

impl Info for Customer {
    fn info(&self) {
        println!("customer _ {}", self.name);
    }
}

// only one function on runtime
// there is another lookup, so it is a bit slow than static dispatcher
fn dynamic_dispatch(t: &dyn Info) {
    t.info()
}

// for this scenario, dyn dispatcher is better
fn dynamic_dispatch_vec(t: Vec<&dyn Info>) {
    for i in t {
        i.info();
    }
}
