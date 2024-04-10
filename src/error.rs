use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use sqlx::error::DatabaseError;
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
    Database(#[from] sqlx::Error),

    #[error("{0}")]
    HashPasswd(#[from] argon2::password_hash::Error),
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

            Error::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, 50002),

            Error::HashPasswd(_) => (StatusCode::INTERNAL_SERVER_ERROR, 50004),
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

pub trait ResultExt<T> {
    /// If `self` contains a SQLx database constraint error with the given name,
    /// transform the error.
    /// Otherwise, the result is passed through unchanged.
    fn on_constraint(
        self,
        name: &str,
        f: impl FnOnce(Box<dyn DatabaseError>) -> Error,
    ) -> core::result::Result<T, Error>;
}

impl<T, E> ResultExt<T> for core::result::Result<T, E>
where
    E: Into<Error>,
{
    fn on_constraint(
        self,
        name: &str,
        map_err: impl FnOnce(Box<dyn DatabaseError>) -> Error,
    ) -> core::result::Result<T, Error> {
        self.map_err(|e| match e.into() {
            Error::Database(sqlx::Error::Database(dbe)) if dbe.constraint() == Some(name) => {
                map_err(dbe)
            }
            e => e,
        })
    }
}
