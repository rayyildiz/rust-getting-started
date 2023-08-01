use banner::print_banner;
use std::collections::HashMap;

fn main() {
    print_banner();

    let mut products = HashMap::new();
    products.insert("p1", vec![1, 2, 3, 4, 7]);
    products.insert("p2", vec![5, 4, 2, 1]);
    products.insert("p3", vec![3, 5, 6, 2, 1]);

    for (product, scores) in products {
        println!(
            "{} , increasing or decreasing {}",
            product,
            popularity_analysis(scores)
        );
    }
}

fn popularity_analysis(scores: Vec<i32>) -> bool {
    let mut increasing = true;
    let mut decreasing = true;

    for i in 0..scores.len() - 1 {
        if scores[i] > scores[i + 1] {
            increasing = false;
        }

        if scores[i] < scores[i + 1] {
            decreasing = false
        }
    }

    increasing || decreasing
}
