pub mod response;
pub mod routes;

#[derive(Clone)]
pub struct ApiState {}

pub async fn launch() {
    let state = ApiState {};
    let routes = routes::routes(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, routes).await.unwrap();
}
