use std::time::Duration;

async fn fetch_data(url: &str) -> Result<String, &'static str> {
    println!("Fetching data from: {}", url);

    tokio::time::sleep(Duration::from_secs(2)).await;

    Ok(String::from(r#"{"name":"rayyildiz","bio":"just for fun"}"#))
}

#[tokio::main]
async fn main() {
    match fetch_data("https://api.rayyildiz.com").await {
        Ok(data) => println!("Received data: {}", data),
        Err(e) => println!("An error occurred: {}", e),
    }
}
