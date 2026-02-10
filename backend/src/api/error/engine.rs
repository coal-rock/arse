use crate::api::error::ApiError;

pub fn unable_to_acquire_lock() -> ApiError {
    ApiError {
        success: false,
        message: Some(String::from("unable to acquire lock")),
    }
}
