use axum::{Router, routing::get};

use crate::helper::query::{handler_hello, handler_hello2};

pub fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2))
    }