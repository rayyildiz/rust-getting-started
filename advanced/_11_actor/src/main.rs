use _11_actor::{Command, Msg};
use actix::Actor;
use actix_rt::System;
use banner::print_banner;

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

    System::current().stop();
}
