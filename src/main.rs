fn main() {

    let s1 = String::from("hello");
    {
        let s2 = &s1 ;
        println!("s1: {}, s2:{}",s1,s2);
    }
    println!("s1: {}",s1);

    let mut v1 = vec![1,2];
    let v2 = v1.clone();
    v1.push(2);
    println!("v2 {:?} ",v2);

}