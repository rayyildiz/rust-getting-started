fn main() {

    let mut v = vec![10, 12, 33, 4, 5, 6, 7];
    v.push(2);
    println!("vector {:?}",v);
    let c = v[5];
    println!("vector 10. {}",c);

    let fruits = vec!["Banana","Apple","Orange"];
    let cities = vec!["Unknown";81];

    let subset_vector = &v[0..=3];
    println!("subset vec: {:?}",subset_vector);



}