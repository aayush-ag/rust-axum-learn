use axum::{Json, routing::post, Router};
use serde::Deserialize;
use serde_json::{Value, json};

use crate::{Error, Result};
#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login", "HANDLER");
    // TODO: Implement real db/auth logic.

    if payload.username != "demo1" || payload.pwd != "welcome" {
        return Err(Error::LoginFail);
    }

    let body = Json(json!({
        "result": {
            "success": true
        }
    }));
    
    Ok(body)
}

