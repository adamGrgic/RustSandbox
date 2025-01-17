use actix_web::{web, App, HttpServer, Responder};
use reqwest::Client;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
struct PostData {
    title: String,
    body: String,
    userId: u32,
}

async fn hello() -> impl Responder {
    "Hello, world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let url = "localhost:8001";

    // Create a client
    let client = Client::new();

    // Create the JSON payload
    let payload = PostData {
        title: "Hello, World!".into(),
        body: "This is a test post.".into(),
        userId: 1,
    };

    // Send a POST request
    let response = client.post(url)
        .json(&payload)
        .send()
        .await?;

    // Parse the response as JSON
    let json: serde_json::Value = response.json().await?;
    println!("Response JSON: {}", json);

    Ok(())
}
