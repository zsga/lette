use axum::{response::IntoResponse, Json};

pub async fn health_check_handler() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "success",
    }))
}

pub async fn not_found_handler() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "failure",
        "message": "bad request"
    }))
}
