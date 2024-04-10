use axum::{response::IntoResponse, Json};

pub mod healthcheck;
pub mod v1;

pub async fn not_found_handler() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "failure",
        "message": "bad request"
    }))
}
