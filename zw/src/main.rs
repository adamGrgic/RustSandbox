use sqlx::sqlite::SqlitePool;
// the goal of this learning project is to create a cli application for tracking game stats in a
// game called zerg wars, which is a custom mod in another game called Starcraft2

use std::io;

#[tokio::main]
async fn main()->Result<(), sqlx::Error> {

    let database_url = "sqlite://example.db";

    let pool = SqlitePool::connect(database_url).await?;

    println!("Successfully connected to the SQLite database!");

    println!("Welcome to Zerg Wars CLI!");
    println!("Enter a profile name");
    let mut profile = String::new();

    io::stdin()
    .read_line(&mut profile)
    .expect("Failed to read line.");

    println!("Accessing profile {}",profile);

    let connection = sqlite::open(":memory:").unwrap();

    let query = "
        CREATE TABLE users (name TEXT, age INTEGER);
        INSERT INTO users VALUES ('Alice', 42);
        INSERT INTO users VALUES ('Bob', 69);
    ";
    connection.execute(query).unwrap();
}
