use tokio::time::{sleep, Duration};

// Simulate a network request
async fn fetch_data(url: &str) -> Result<String, &'static str> {
    println!("Fetching data from: {}", url);

    // business logic

    // In a real application, you would make an actual network request here
    // For the sake of this example, we'll just return a fixed response
    Ok(String::from(r#"{"name":"rayyildiz","bio":"just for fun"}"#))
}

#[tokio::main]
async fn main() {
    match fetch_data("https://api.rayyildiz.com").await {
        Ok(data) => println!("Received data: {}", data),
        Err(e) => println!("An error occurred: {}", e),
    }
}
