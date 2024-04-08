use std::sync::Arc;

use crate::{
    api::{healthcheck::health_check_handler, not_found_handler, v1::ApiDoc},
    conf::Conf,
};
use axum::{routing::get, Router};
use sqlx::MySqlPool;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub struct AppState {
    pub pool: MySqlPool,
}

pub async fn new(conf: &Conf) -> Router {
    let app_state = Arc::new(AppState {
        pool: get_connect_pool(conf).await,
    });

    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .nest("/api", api_router())
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
        .with_state(Arc::clone(&app_state))
}

pub fn api_router() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/v1", v1_router())
        .fallback(not_found_handler)
}

pub fn v1_router() -> Router<Arc<AppState>> {
    Router::new().route("/healthcheck", get(health_check_handler))
}

pub async fn serve(conf: &Conf, port: u16) -> anyhow::Result<()> {
    let app = new(conf).await;
    let addr = format!("{}:{}", conf.app.host, port);
    let listener = TcpListener::bind(addr).await?;

    tracing::info!("Listening on: http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await?;

    Ok(())
}

pub async fn get_connect_pool(conf: &Conf) -> MySqlPool {
    MySqlPool::connect_with(conf.db.with_db())
        .await
        .expect("Failed to connect MySql")
}
