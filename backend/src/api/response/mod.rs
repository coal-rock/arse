pub mod admin;

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

use crate::api::error::ApiError;

#[derive(Serialize, ts_rs::TS)]
#[ts(export)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: T,
}

impl<T> From<T> for ApiResponse<T>
where
    T: Serialize,
{
    fn from(value: T) -> Self {
        Self {
            success: true,
            data: value,
        }
    }
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

pub type ApiResult<T> = Result<ApiResponse<T>, ApiError>;
