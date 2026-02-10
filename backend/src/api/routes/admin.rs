use axum::routing::{get, post};
use axum::{Router, extract::State};

use crate::api::error::engine::unable_to_acquire_lock;
use crate::api::response::ApiResult;
use crate::api::response::admin::CheckSchemaResponse;
use crate::api::routes::ApiState;
use crate::engine::checks::get_available_checks;

pub fn routes<S>(state: ApiState) -> Router<S> {
    Router::new()
        .route("/listAvailableChecks", get(list_available_checks))
        .route("/startEngine", post(start_engine))
        .route("/stopEngine", post(stop_engine))
        .with_state(state)
}

async fn list_available_checks(State(_): State<ApiState>) -> ApiResult<Vec<CheckSchemaResponse>> {
    ApiResult::Ok(
        get_available_checks()
            .iter()
            .cloned()
            .map(|c| c.into())
            .collect::<Vec<CheckSchemaResponse>>()
            .into(),
    )
}

async fn start_engine(State(state): State<ApiState>) -> ApiResult<()> {
    match &mut state.engine.lock() {
        Ok(engine) => {
            engine.start();
            ApiResult::Ok(().into())
        }
        Err(_) => ApiResult::Err(unable_to_acquire_lock()),
    }
}

async fn stop_engine(State(state): State<ApiState>) -> ApiResult<()> {
    match &mut state.engine.lock() {
        Ok(engine) => {
            engine.start();
            ApiResult::Ok(().into())
        }
        Err(_) => ApiResult::Err(unable_to_acquire_lock()),
    }
}
