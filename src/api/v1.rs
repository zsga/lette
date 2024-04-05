use super::healthcheck;
use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        healthcheck::health_check_handler,
    ),
    components(
        schemas(
            crate::api::error::ErrorResponse,
            crate::api::error::Status,
        ),
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "healthcheck", description = "Health Check"),
    ),
    servers(
        (url = "/", description = "API Server")
    )
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "api_jwt_token",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        )
    }
}
