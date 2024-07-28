use tokio;
use tokio::time::{interval, Duration};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .enable_io()
        .build()
        .unwrap()
        .block_on(poll())
}

async fn poll() -> Result<(), Box<dyn std::error::Error>> {
    let mut interval = interval(Duration::from_secs(2));
    let client = reqwest::Client::new();
    loop {
        interval.tick().await;
        let response = client.get("https://google.com").send().await?;
        println!("status: {}", response.status());
    }
}
