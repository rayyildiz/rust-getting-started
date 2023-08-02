use banner::print_banner;

fn main() {
    print_banner();

    let mut s = MaxStack::new();
    s.push(140);
    s.push(80);
    s.push(55);
    s.push(120);
    s.push(145);

    println!("max {:?}", s.max_stack);
    s.pop();
    println!("max2  {:?}", s.max_stack);
}

#[derive(Debug)]
struct MaxStack {
    max_stack: Vec<i32>,
    main_stack: Vec<i32>,
}

impl MaxStack {
    fn new() -> Self {
        MaxStack {
            main_stack: Vec::new(),
            max_stack: Vec::new(),
        }
    }

    fn push(&mut self, v: i32) {
        self.main_stack.push(v);
        if !self.max_stack.is_empty() && self.max_stack.last().unwrap() > &v {
            // self.max_stack.push(v);
            self.max_stack.push(*self.max_stack.last().unwrap());
        } else {
            self.max_stack.push(v);
        }
    }

    fn pop(&mut self) {
        self.main_stack.pop();
        self.max_stack.pop();
    }

    fn max_value(&self) -> i32 {
        *self.max_stack.last().unwrap()
    }
}
