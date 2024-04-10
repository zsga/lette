use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use utoipa::ToSchema;

pub struct ErrorResponse {
    pub code: String,
    pub message: String,
}

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(thiserror::Error, Debug, ToSchema)]
pub enum Error {
    #[error("{0}")]
    BadRequest(#[from] BadRequest),

    #[error("{0}")]
    NotFound(#[from] NotFound),

    #[error("{0}")]
    Auth(#[from] AuthError),

    #[error("{0}")]
    HashPasswd(#[from] argon2::Error),
}

impl Error {
    fn get_code(&self) -> (StatusCode, u16) {
        match *self {
            // 4xx erros
            Error::BadRequest(_) => (StatusCode::BAD_REQUEST, 40002),
            Error::NotFound(_) => (StatusCode::NOT_FOUND, 40003),

            Error::Auth(AuthError::WrongCredentials) => (StatusCode::UNAUTHORIZED, 40004),
            Error::Auth(AuthError::InvalidToken) => (StatusCode::UNAUTHORIZED, 40005),
            Error::Auth(AuthError::Locked) => (StatusCode::LOCKED, 40006),

            // 5xx errors
            Error::Auth(AuthError::TokenCreation) => (StatusCode::INTERNAL_SERVER_ERROR, 50001),

            Error::HashPasswd(_) => (StatusCode::INTERNAL_SERVER_ERROR, 50002),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let (status_code, error_code) = self.get_code();
        let body = Json(json!({
            "code": error_code, "message": self.to_string()
        }));
        (status_code, body).into_response()
    }
}

#[derive(thiserror::Error, Debug)]
#[error("Bad Request {message}")]
pub struct BadRequest {
    message: String,
}

#[derive(thiserror::Error, Debug)]
#[error("Not found")]
pub struct NotFound {}

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("Wrong authentication credentials")]
    WrongCredentials,
    #[error("Failed to create authentication token")]
    TokenCreation,
    #[error("Invalid authentication credentials")]
    InvalidToken,
    #[error("User is locked")]
    Locked,
}
