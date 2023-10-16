use banner::print_banner;

fn main() {
    print_banner();

    let kmh = Kmh { value: 90 };
    let km = kmh.distance_in_three_hours();

    println!("Kmh : {:?} ", km);

    let mph = Mph { value: 60 };
    let mp = mph.distance_in_three_hours();

    println!("Mph : {:?} ", mp);

    let a = find_sample();
    let b = a.distance_in_three_hours();
    println!("b {:?}", b);
}

#[derive(Debug)]
struct Kmh {
    value: u32,
}

#[derive(Debug)]
struct Km {
    value: u32,
}

#[derive(Debug)]
struct Mph {
    value: u32,
}

#[derive(Debug)]
struct Mp {
    value: u32,
}

trait DistanceInThreeHours {
    type Distance;

    fn distance_in_three_hours(&self) -> Self::Distance;
}

impl DistanceInThreeHours for Kmh {
    type Distance = Km;

    fn distance_in_three_hours(&self) -> Self::Distance {
        Self::Distance {
            value: self.value * 3,
        }
    }
}

impl DistanceInThreeHours for Mph {
    type Distance = Mph;

    fn distance_in_three_hours(&self) -> Self::Distance {
        Self::Distance {
            value: self.value * 3,
        }
    }
}

struct Milli(u16);

impl DistanceInThreeHours for Milli {
    type Distance = i32;

    fn distance_in_three_hours(&self) -> Self::Distance {
        self.0 as i32 + 10
    }
}

fn find_sample() -> impl DistanceInThreeHours<Distance = i32> {
    Milli(10)
}

/*

original version. use associated type

impl Kmh {
    fn distance_in_three_hours(&self) -> Km {
        Km {
            value: self.value * 3,
        }
    }
}

impl Mph {
    fn distance_in_three_hours(&self) -> Mp {
        Mp {
            value: self.value * 3,
        }
    }
}
*/
