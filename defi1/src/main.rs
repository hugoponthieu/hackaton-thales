use reqwest::Client;
use serde_json::json;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let res = client
        .post("https://rust-checker.hackathon.dopolytech.fr/check")
        .body(
            json!({
                "content": "fn main() { println!(\"Hello, world!\"); }",
            })
            .to_string(),
        )
        .send()
        .await?;
    let body = res.text().await?;
    print!("{}", body);
    Ok(())
}
