use axum::Json;
use axum::routing::{get, post};
use axum::{Router, extract::State};
use serde::Deserialize;

use crate::api::response::ApiResult;
use crate::api::response::admin::{CheckSchemaResponse, EngineStatusResponse};
use crate::api::routes::ApiState;
use crate::db;
use crate::engine::checks::get_available_checks;

pub fn routes<S>(state: ApiState) -> Router<S> {
    Router::new()
        .route("/listAvailableChecks", get(list_available_checks))
        .route("/startEngine", post(start_engine))
        .route("/stopEngine", post(stop_engine))
        .route("/getEngineStatus", get(get_engine_status))
        .route("/createAdminAccount", post(create_initial_admin_account))
        .with_state(state)
}

async fn list_available_checks(State(_): State<ApiState>) -> ApiResult<Vec<CheckSchemaResponse>> {
    ApiResult::Ok(
        get_available_checks()
            .iter()
            .clone()
            .map(|c| c.into())
            .collect::<Vec<CheckSchemaResponse>>()
            .into(),
    )
}

async fn start_engine(State(state): State<ApiState>) -> ApiResult<()> {
    state.engine.lock().await.start();
    ApiResult::Ok(().into())
}

async fn stop_engine(State(state): State<ApiState>) -> ApiResult<()> {
    state.engine.lock().await.stop();
    ApiResult::Ok(().into())
}

async fn get_engine_status(State(state): State<ApiState>) -> ApiResult<EngineStatusResponse> {
    ApiResult::Ok(
        EngineStatusResponse {
            running: state.engine.lock().await.is_running(),
        }
        .into(),
    )
}

#[derive(Deserialize)]
struct CreateAdminAccountRequest {
    username: String,
    password: String,
}

async fn create_initial_admin_account(
    State(state): State<ApiState>,
    Json(payload): Json<CreateAdminAccountRequest>,
) -> ApiResult<()> {
    let pool = &state.engine.lock().await.db_pool;

    if db::users::does_admin_user_exist(pool.clone()).await {
        return ApiResult::Err("admin user already exists".into());
    }

    db::users::add_user(pool.clone(), payload.username, payload.password).await;
    return ApiResult::Ok(().into());
}
