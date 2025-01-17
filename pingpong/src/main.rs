use reqwest::Client;
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
struct PostData {
    title: String,
    body: String,
    user_id: u32,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {

    let url = "localhost:8001";

    // Create a client
    let client = Client::new();

    // Create the JSON payload
    let payload = PostData {
        title: "Hello, World!".into(),
        body: "This is a test post.".into(),
        user_id: 1,
    };

    // Send a POST request
    let response = client.post(url)
        .json(&payload)
        .send()
        .await?;


    // Parse the response as JSON
    let json: Value = response.json().await?;
    println!("Response JSON: {}", json);

    Ok(())
}
