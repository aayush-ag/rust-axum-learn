use axum::{Router, routing::{get, get_service}};
use tower_http::services::ServeDir;

use crate::helper::query::{handler_hello, handler_hello2};

pub fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2))
    }

pub fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}