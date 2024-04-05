use axum::{routing::get, Router};

use crate::handler::{health_check_handler, not_found_handler};

pub fn new() -> Router {
    Router::new()
        .route("/api/healthcheck", get(health_check_handler))
        .fallback(not_found_handler)
}
