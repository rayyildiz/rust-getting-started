use std::f32::consts::PI;

struct Circle {
    rad: f32,
}

impl Circle {
    fn area(&self) -> f32 {
        (self.rad * self.rad) * PI
    }

    fn contain(&self, another: &Circle) -> bool {
        let mine = self.area();
        let other = another.area();

        mine > other
    }
}

fn throw_panic() {
    panic!("panic panic");
}

fn square(a: f32) -> f32 {
    a * a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2, 1 + 1);
    }

    #[test]
    fn larger_circle_can_contain_small() {
        let large = Circle { rad: 50.0 };
        let small = Circle { rad: 45.0 };

        assert!(large.area() > small.area());
    }

    #[test]
    fn small_circle_can_not_contain_large() {
        let large = Circle { rad: 5.0 };
        let small = Circle { rad: 4.0 };

        assert!(
            !small.contain(&large),
            "this fails because small cannot contain a bigger circle"
        );
    }

    #[test]
    #[should_panic(expected = "panic panic")]
    fn should_panic() {
        throw_panic();
    }

    #[test]
    fn test_result() -> Result<(), String> {
        if 3 + 3 == 6 {
            Ok(())
        } else {
            Err("error".to_string())
        }
    }

    #[test]
    fn test_square() {
        assert_eq!(9.0, square(3.0))
    }

    #[test]
    #[ignore]
    fn test_ignore() {}
}
