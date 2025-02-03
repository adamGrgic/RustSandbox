use axum::{
    routing::{get, post},
    extract::{Form, State},
    response::Html,
    Router,
};
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
struct AppState {
    tasks: Arc<RwLock<Vec<String>>>,
}

#[tokio::main]
async fn main() {
    let state = AppState {
        tasks: Arc::new(RwLock::new(vec![])),
    };

    let app = Router::new()
        .route("/", get(show_todo_list))
        .route("/add-task", post(add_task))
        .with_state(state);

    println!("🚀 Running on http://127.0.0.1:3000/");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn show_todo_list(State(state): State<AppState>) -> Html<String> {
    let tasks = state.tasks.read().await;
    let task_items: String = tasks
        .iter()
        .enumerate()
        .map(|(idx, task)| format!(r#"<li>{task} <button hx-delete="/remove-task/{idx}" hx-swap="outerHTML">❌</button></li>"#))
        .collect();

    let html = format!(
        r#"
        <!DOCTYPE html>
        <html lang='en'>
        <head>
            <meta charset='UTF-8'>
            <meta name='viewport' content='width=device-width, initial-scale=1.0'>
            <title>Rust HTMX Todo</title>
            <script src="https://unpkg.com/htmx.org@1.9.5"></script>
        </head>
        <body>
            <h1>HTMX Todo App</h1>
            <form hx-post='/add-task' hx-target='#todo-list' hx-swap='beforeend'>
                <input type='text' name='task' required>
                <button type='submit'>Add Task</button>
            </form>
            <ul id='todo-list'>{task_items}</ul>
        </body>
        </html>
    "#
    );

    Html(html)
}

#[derive(Deserialize)]
struct TaskForm {
    task: String,
}

async fn add_task(State(state): State<AppState>, Form(form): Form<TaskForm>) -> Html<String> {
    let mut tasks = state.tasks.write().await;
    tasks.push(form.task.clone());

    Html(format!(
        r#"<li>{} <button hx-delete="/remove-task/{}" hx-swap="outerHTML">❌</button></li>"#,
        form.task, tasks.len() - 1
    ))
}

