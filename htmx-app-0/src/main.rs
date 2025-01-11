use axum::{
    routing::{get},
    extract::{Form, Extension},
    response::{Html, IntoResponse},
    Router,
};
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    // Create shared state: a list of items
    let items = Arc::new(Mutex::new(Vec::<String>::new()));

    // Create the router
    let app = Router::new()
        .route("/", get(render_index))
        .layer(Extension(items)); // Attach the shared state

    // Start the server
    println!("Server running at http://localhost:3000");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Render the index page
async fn render_index() -> impl IntoResponse {
    let html = r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name='viewport' content="width=device-width, initial-scale=1.0">
        <title>Rust HTMX Demo</title>
        <script src="https://unpkg.com/htmx.org"></script>
    </head>
    <body>
        <h1>Item List</h1>
        <ul id="item-list"></ul>
        <form hx-post="/add_item" hx-target='#item-list' hx-swap='beforeend'>
            <input type="text" name="item" placeholder="Add an item" required>
            <button type="submit">Add</button>
        </form>
    </body>
    </html>
    "#;

    Html(html)
}

