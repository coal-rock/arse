pub mod api;
pub mod config;
pub mod engine;

#[tokio::main]
async fn main() {
    api::launch().await;
}
