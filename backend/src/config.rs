use crate::engine::team::Team;
use std::net::IpAddr;

pub struct Config {
    pub target_round_time: f32,
    pub interface: IpAddr,
    pub port: u16,
    pub max_concurrect_checks: u32,
    pub admin_username: String,
    pub admin_password: String,
    pub teams: Vec<Team>,
}
