use banner::print_banner;
use std::fmt::{Debug, Display, Formatter};
use std::future::Future;
use std::ops::Deref;
use std::path::Iter;

fn main() -> Result<(), String> {
    print_banner();

    let mut s = Stack::new(10);

    s.push(12)?;
    s.push(10)?;
    s.push(78)?;
    s.push(1)?;
    s.push(4)?;

    println!("s : {:?}", s);
    println!("pop : {:?}", s.pop());
    println!("current : {:?}", s);

    Ok(())
}

#[derive(Debug)]
struct Stack<T> {
    elems: Vec<T>,
    _count: usize,
}

impl<T> Stack<T> {
    fn new(max_size: usize) -> Self {
        Stack {
            elems: Vec::with_capacity(max_size),
            _count: 0,
        }
    }

    fn push(&mut self, e: T) -> Result<(), String> {
        if self.elems.capacity() <= self._count {
            Err("can not add more".to_string())
        } else {
            self.elems.push(e);
            self._count += 1;

            Ok(())
        }
    }

    fn pop(&mut self) -> Option<T> {
        self.elems.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.elems.last()
    }
}
