use crate::api::{healthcheck::health_check_handler, not_found_handler, v1::ApiDoc};
use axum::{routing::get, Router};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub fn new() -> Router {
    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .nest("/api", api_router())
}

pub fn api_router() -> Router {
    Router::new()
        .nest("/v1", v1_router())
        .fallback(not_found_handler)
}

pub fn v1_router() -> Router {
    Router::new().route("/healthcheck", get(health_check_handler))
}
