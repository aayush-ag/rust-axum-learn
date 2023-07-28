mod helper;
mod error;
mod web;

use axum::{
    routing::get,
    Router,
};
use helper::routes::{routes_hello, routes_static};
use self::error::{Error, Result};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }))
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .fallback_service(routes_static());
    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
