mod helper;

use axum::{
    routing::get,
    Router,
};
use helper::query::handler_hello;
#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }))
    .route("/hello", get(handler_hello));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
