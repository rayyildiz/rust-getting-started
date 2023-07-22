fn main() {

    let mut v = vec![1,2,3,4];
    let ref1 = &v;
    let ref2 = &v;

    println!("r1: {:?} r2: {:?}", ref1, ref2);
}