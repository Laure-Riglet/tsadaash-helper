/* mod cli;
mod domain;
mod db;

use cli::menu::init_menu;
use db::connection::connect;
use rusqlite::{ Connection };

fn main() -> rusqlite::Result<()> {

    // Establish & set database connection
    let conn: Connection = connect()?;

    // Start CLI menu loop
    init_menu(&conn)?;

    Ok(())
} */

use axum::extract::{Json, Path, Query};
use axum::{routing::get, Router};
use std::collections::HashMap;

#[tokio::main]
async fn main() {

    // our router
    let app: Router<()> = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/foo", get(get_foo).post(post_foo))
        .route("/foo/bar", get(foo_bar))
        .route("/path/{user_id}", get(path))
        // http://localhost:3000/query?foo=1&bar=2
        .route("/query", get(query))
        .route("/json", get(json));

    // BASIC HANDLERS

    // which calls one of these handlers
    async fn get_foo() -> &'static str {
        "Hello, Foo!"
    }
    async fn post_foo() -> &'static str {
        "Posted to Foo!"
    }
    async fn foo_bar() -> &'static str {
        "Hello from Foo Bar!"
    }

    // HANDLERS WITH EXTRACTORS

    // `Path` gives you the path parameters and deserializes them.
    async fn path(Path(user_id): Path<u32>) -> String {
        format!("user_id is {}", user_id) // use the user_id variable here
    }

    // `Query` gives you the query parameters and deserializes them.
    async fn query(Query(params): Query<HashMap<String, String>>) {
        for (key, value) in params.iter() {
            println!("{}: {}", key, value);
        }
    }

    // Buffer the request body and deserialize it as JSON into a
    // `serde_json::Value`. `Json` supports any type that implements
    // `serde::Deserialize`.
    async fn json(Json(payload): Json<serde_json::Value>) {
        println!("payload: {}", payload);
    }

    // HANDLER WITH RESPONSES

    // LISTEN AND SERVE THE APP

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
