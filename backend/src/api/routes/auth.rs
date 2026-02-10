use axum::routing::get;
use axum::{Router, extract::State};

use crate::api::routes::ApiState;

pub fn routes<S>(state: ApiState) -> Router<S> {
    Router::new()
        .route("/", get(|_: State<ApiState>| async {}))
        .with_state(state)
}
