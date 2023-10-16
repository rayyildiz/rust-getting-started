use banner::print_banner;
use std::time::Instant;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    print_banner();

    let start = Instant::now();
    let (tx, mut rx) = mpsc::channel(100);

    for i in 0..1_000 {
        let tx = tx.clone();

        tokio::spawn(async move {
            tx.send(format!("{i} processing"))
                .await
                .expect("failed to send msg");
        });
    }

    drop(tx);

    while let Some(a) = rx.recv().await {
        // println!("get data {a}")
    }

    let duration = start.elapsed();
    println!("elapsed {:?}", duration);
}
