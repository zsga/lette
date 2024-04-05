use axum::{response::IntoResponse, Json};

#[utoipa::path(
    get,
    path = "/api/v1/healthcheck",
    responses(
        (status = 200, description = "Health Check", body = Json)
    )
)]
pub async fn health_check_handler() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "success",
    }))
}
