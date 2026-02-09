use crate::engine::team::Team;
use std::net::IpAddr;

struct Config {
    target_round_time: f32,
    interface: IpAddr,
    port: u8,
    max_concurrect_checks: u32,
    admin_username: String,
    admin_password: String,
    teams: Vec<Team>,
}
