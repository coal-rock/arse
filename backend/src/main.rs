use std::{net::Ipv4Addr, path::PathBuf};

use crate::{
    config::Config,
    engine::{Engine, checks::random::RandomCheck, service::Service, team::Team},
};

pub mod api;
pub mod config;
pub mod db;
pub mod engine;

#[tokio::main]
async fn main() {
    let config = Config {
        target_round_time: 10.0,
        interface: Ipv4Addr::new(0, 0, 0, 0).into(),
        port: 3000,
        max_concurrect_checks: 10,
        admin_username: "admin".into(),
        admin_password: "bb123#123".into(),
        database_path: PathBuf::from("arse.db"),
        teams: vec![Team {
            name: "Team 1".into(),
            services: vec![Service {
                name: "RandomCheck".into(),
                weight: 100,
                check: Box::new(RandomCheck::new(50.0)),
            }],
        }],
    };

    let (interface, port) = (config.interface, config.port);
    let engine = Engine::new(config).await;

    let _ = tokio::join!(tokio::spawn(api::launch(interface, port, engine)));
}
