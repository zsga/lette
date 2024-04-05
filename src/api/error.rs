use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct ErrorResponse {
    pub status: Status,
    pub message: String,
}

#[derive(Serialize, ToSchema)]
pub enum Status {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "failure")]
    Failure,
}
