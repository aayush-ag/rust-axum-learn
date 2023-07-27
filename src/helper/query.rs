use axum::{response::{IntoResponse, Html}, extract::{Query, Path}};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct HelloParams {
    name: Option<String>,
}
pub async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {

    println!("->> {:<12} - handler_hello", "HANDLER");
    let name = params.name.as_deref().unwrap_or("World!");
    Html(format!("Hello <strong>{name}!!!</strong>"))
}

pub async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {

    println!("->> {:<12} - handler_hello2 - {name:?}", "HANDLER");
    Html(format!("Hello2 <strong>{name}!!!</strong>"))
}
