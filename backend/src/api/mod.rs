use std::{net::IpAddr, sync::Arc};
use tokio::sync::Mutex;

use crate::engine::Engine;

pub mod error;
pub mod middleware;
pub mod response;
pub mod routes;

#[derive(Clone)]
pub struct ApiState {
    engine: Arc<Mutex<Engine>>,
}

pub async fn launch(interface: IpAddr, port: u16, engine: Engine) {
    let state = ApiState {
        engine: Arc::new(Mutex::new(engine)),
    };
    let routes = routes::routes(state);

    let listener = tokio::net::TcpListener::bind(format!("{}:{port}", interface.to_string()))
        .await
        .unwrap();

    axum::serve(listener, routes).await.unwrap();
}
