fn main() {
    let v = vec![1, 2, 3, 4];
    let mut it = v.iter();

    println!("iterator :{:?}", it);
    println!("it {:?}", it.next());
    println!("it {:?}", it.next());
    println!("it {:?}", it.next());
    println!("it {:?}", it.next());
    println!("it {:?}", it.next());

    let v = vec![1, 2, 4, 3, 10, 5, 6, 7, 8, 9];
    let check = v.iter().any(|&a| a == 7);
    println!(" [{:?}] has any 7 ? {}", v, check);

    let all_positive = v.iter().all(|&a| a > 0);
    println!(" [{:?}] all positive ? {}", v, all_positive);

    let find_evens = v.iter().find(|&a| a % 2 == 0);
    println!(" [{:?}] first even number ? {:?}", v, find_evens);

    let find_evens = v.iter().rfind(|&a| a % 2 == 0);
    println!(" [{:?}] last even number ? {:?}", v, find_evens);

    let max = v.iter().max();
    println!(" [{:?}] max value {:?}", v, max);

    let min = v.iter().min();
    println!(" [{:?}] min value {:?}", v, min);

    let sum: i32 = v.iter().sum();
    println!(" [{:?}] sum {:?}", v, sum);

    let evens = v.iter().filter(|&a| *a % 2 == 0).collect::<Vec<&i32>>();
    println!(" [{:?}] evens {:?}", v, evens);


    let v2 = v.clone();
    let evens2 = v2.clone().into_iter().filter(|&a| a % 2 == 0).collect::<Vec<i32>>();
    println!(" [{:?}] evens {:?}",v2, evens2);


    let m = v.iter().map(|a| 1.18 * *a as f32).collect::<Vec<f32>>();
    println!(" [{:?}] new map {:?}",v, m);
}
