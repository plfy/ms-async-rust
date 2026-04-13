use std::time::Duration;

mod chapter1;
mod chapter2;

#[tokio::main]
async fn main() {
    let data = fetch_data().await;
    println!("data: {}", data);
}

async fn fetch_data() -> String {
    // sleep 2 seconds
    tokio::time::sleep(Duration::from_secs(2)).await;
    "ok".to_string()
}
