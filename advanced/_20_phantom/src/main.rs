use std::marker::PhantomData;

use banner::print_banner;

fn main() {
    print_banner();

    let cb = CircuitBreaker::new();
    println!("is is closed ? {}", cb.accept_request());
    let open_cb = cb.open();
    let closed = open_cb.close();
    println!("is is closed again ? {}", closed.accept_request());
}

struct Closed;

struct Open;

struct CircuitBreaker<Status = Closed> {
    token: i32,
    status: PhantomData<Status>,
}

impl CircuitBreaker {
    fn new() -> Self {
        Self {
            token: 0,
            status: PhantomData,
        }
    }
}

impl CircuitBreaker<Closed> {
    fn open(&self) -> CircuitBreaker<Open> {
        CircuitBreaker {
            token: self.token,
            status: PhantomData::<Open>,
        }
    }

    fn accept_request(&self) -> bool {
        true
    }
}

impl CircuitBreaker<Open> {
    fn close(&self) -> CircuitBreaker<Closed> {
        CircuitBreaker {
            token: self.token,
            status: PhantomData::<Closed>,
        }
    }
}
