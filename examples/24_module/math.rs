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

        /// Multiple given a text
        ///
        /// # Tests
        /// ```
        /// let result = math::math::ops::multiple(3,4);
        ///
        /// assert_eq!(result,12);
        /// ```
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

#[cfg(test)]
mod tests {
    use crate::math::math;
    use crate::math::math::ops::Operation::*;

    #[test]
    fn test_add() {
        assert_eq!(7, math::ops::add(3, 4));
        assert_eq!(3.3, math::ops::add(1.0, 2.3));
        assert_eq!(-5, math::ops::add(-1, -4));
        assert_eq!(2.0, math::ops::add(-2.2, 4.2));
    }

    #[test]
    fn test_multiple() {
        assert_eq!(12, math::ops::multiple(3, 4));
        assert_eq!(-4, math::ops::multiple(-1, 4));
        assert_eq!(-8.4, math::ops::multiple(-2.1, 4.0));
        assert_eq!(4.41, math::ops::multiple(2.1, 2.1));
        assert_eq!(3, math::ops::multiple(-1, -3));
    }

    #[test]
    fn test_div() {
        assert_eq!(0, math::ops::divide(3, 4));
        assert_eq!(0.75, math::ops::divide(3.0, 4.0));
        assert_eq!(-1, math::ops::divide(-4, 4));
        assert_eq!(2.1, math::ops::divide(8.4, 4.0));
        assert_eq!(3.0, math::ops::divide(-9.0, -3.0));
        assert_eq!(4, math::ops::divide(-16, -4));
    }

    #[test]
    fn test_command() {
        assert_eq!(7, math::ops::command(Add, 3, 4));
        assert_eq!(6.3, math::ops::command(Add, 2.0, 4.3));
        assert_eq!(12, math::ops::command(Multiple, 3, 4));
        assert_eq!(12.0, math::ops::command(Multiple, 3.0, 4.0));
        assert_eq!(0, math::ops::command(Divide, 3, 4));
        assert_eq!(0.75, math::ops::command(Divide, 3.0, 4.0));
    }
}
