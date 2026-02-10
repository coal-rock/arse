use axum::routing::get;
use axum::{Router, extract::State};

use crate::api::response::ApiResponse;
use crate::api::response::admin::CheckSchemaResponse;
use crate::api::routes::ApiState;
use crate::engine::checks::get_available_checks;

pub fn routes<S>(state: ApiState) -> Router<S> {
    Router::new()
        .route("/listAvailableChecks", get(list_available_checks))
        .with_state(state)
}

async fn list_available_checks(State(_): State<ApiState>) -> ApiResponse<Vec<CheckSchemaResponse>> {
    ApiResponse::ok(
        get_available_checks()
            .iter()
            .cloned()
            .map(|c| c.into())
            .collect::<Vec<CheckSchemaResponse>>(),
    )
}
