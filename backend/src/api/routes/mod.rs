pub mod admin;
pub mod auth;
pub mod scoring;

use crate::api::ApiState;
use axum::Router;

pub fn routes(state: ApiState) -> Router {
    Router::new()
        .nest("/admin", admin::routes(state.clone()))
        .nest("/auth", auth::routes(state.clone()))
        .nest("/scoring", scoring::routes(state.clone()))
}
