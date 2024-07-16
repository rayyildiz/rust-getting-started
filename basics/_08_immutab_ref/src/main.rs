use banner::print_banner;

fn main() {
    print_banner();

    let mut v = vec![1, 2, 3, 4];
    let ref1 = &v;
    let ref2 = &v;

    println!("r1: {:?} r2: {:?}", ref1, ref2);
    v.push(3);

    {
        let r3 = &mut v;
        println!("r3: {:?}", r3);
    }

    let mut v2 = vec![5, 8, 9, 7];

    v2 = receiving_giving_back_ownership(v2);

    let mut numbers = vec![1, 2, 3];

    let slice = get_slice(&mut numbers);

    // numbers.push(4);

    println!("Slice: {:?}", slice);
}

fn receiving_giving_back_ownership(vec1: Vec<i32>) -> Vec<i32> {
    vec1
}

fn get_slice(numbers: &mut Vec<i32>) -> &[i32] {
    &numbers[0..2]
}
