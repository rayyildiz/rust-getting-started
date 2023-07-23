pub mod math {

    pub mod ops {
        pub enum Operation {
            Add,
            Multiple,
            Divide,
        }

        pub fn command<T>(op: Operation, a: T, b: T) -> T
        where
            T: std::ops::Add<Output = T>
                + std::ops::Mul<Output = T>
                + std::ops::Div<Output = T>
                + Copy,
        {
            let result = match op {
                Operation::Add => add(a, b),
                Operation::Multiple => multiple(a, b),
                Operation::Divide => divide(a, b),
            };

            result
        }

        pub fn multiple<T>(a: T, b: T) -> T
        where
            T: std::ops::Mul<Output = T> + Copy,
        {
            a * b
        }

        pub fn add<T>(a: T, b: T) -> T
        where
            T: std::ops::Add<Output = T> + Copy,
        {
            a + b
        }

        pub fn divide<T>(a: T, b: T) -> T
        where
            T: std::ops::Div<Output = T> + Copy,
        {
            a / b
        }

        const VERSION: &str = "0.1.0";

        pub fn print_version() {
            println!("math ops: v{}", VERSION);
        }
    }
}
