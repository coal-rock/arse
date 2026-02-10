pub mod engine;

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiError {
    pub success: bool,
    pub message: Option<String>,
}

impl From<&str> for ApiError {
    fn from(value: &str) -> Self {
        Self {
            success: false,
            message: Some(value.to_string()),
        }
    }
}

// TODO: this might not be the best status code to return here.
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (StatusCode::BAD_REQUEST, Json(self)).into_response()
    }
}
