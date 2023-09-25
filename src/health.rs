use axum::{response::IntoResponse, Json, Router, routing::get};
use serde_json::json;

pub fn health_router() ->Router {
    Router::new()
    .route("/api/health", get(health_checker_handler))
}

pub async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Axum & SQLX & POSTGRES";

    let json_response = json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}