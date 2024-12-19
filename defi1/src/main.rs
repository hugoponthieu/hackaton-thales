use std::collections::HashMap;
use reqwest::Client;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut map = HashMap::new();
    map.insert("content", "rust");
    let client = Client::new();
    let res = client
        .post("https://rust-checker.hackathon.dopolytech.fr/check")
        .json(&map)
        .send()
        .await?;
    let body = res.text().await?;
    print!("{}", body);
    Ok(())
}
