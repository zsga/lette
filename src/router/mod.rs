use crate::api::{healthcheck::health_check_handler, not_found_handler, v1::ApiDoc};
use axum::{routing::get, Router};
use tokio::net::TcpListener;
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

pub async fn serve(port: u16) -> anyhow::Result<()> {
    let app = new();
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(addr).await?;

    tracing::info!("Listening on: http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await?;

    Ok(())
}
