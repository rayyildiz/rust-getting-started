use _11_actor::{Command, Msg};
use actix::Actor;
use actix_rt::System;
use banner::print_banner;
use std::future::Future;

#[actix_rt::main]
async fn main() {
    print_banner();

    println!("starting actor system");

    let m = Msg {
        text: "hello world".to_string(),
    }
    .start();

    let response = m.send(Command::LangDetect).await.expect("can not send msg");
    println!("response1 {response:?}");

    let response = m
        .send(Command::SentimentAnalysis)
        .await
        .expect("can not sentiment analysis");
    println!("response2 {response:?}");

    let it = vec![true, true, false];
    let a = filter_by_true(it.into_iter()).await;

    for abc in a {
        println!("res :{abc}");
    }

    System::current().stop();
}

fn filter_by_true<I>(iter: I) -> impl Future<Output = impl Iterator<Item = bool>>
where
    I: Iterator<Item = bool>,
{
    async move { iter.filter(|&x| x) }
}
