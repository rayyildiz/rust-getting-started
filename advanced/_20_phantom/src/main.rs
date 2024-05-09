use std::marker::PhantomData;

use banner::print_banner;

fn main() {
    print_banner();

    let cb = CircuitBreaker::new();
    let open_cb = cb.open();
    open_cb.close();
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
            status: PhantomData::default(),
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
