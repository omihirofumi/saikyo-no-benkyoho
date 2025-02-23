mod handlers;

use axum::{
    routing::{get, post},
    Router,
};
use handlers::{
    hello::{echo, string_handler},
    record::create_record,
};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/echo", get(string_handler))
        .route("/records", post(create_record));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
