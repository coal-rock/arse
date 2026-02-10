pub mod checks;
pub mod service;
pub mod team;

use crate::{
    config::Config,
    engine::{
        checks::check::{CheckError, CheckResult},
        service::Service,
        team::Team,
    },
};
use std::{thread, time};

pub struct Engine {
    running: bool,
    config: Config,
}

impl Engine {
    pub fn new(config: Config) -> Self {
        Self {
            running: false,
            config: config,
        }
    }

    pub fn start(&mut self) {
        self.running = true;
    }

    pub fn stop(&mut self) {
        self.running = false;
    }

    pub fn spawn(&mut self) {
        loop {
            if !self.running {
                thread::sleep(time::Duration::from_secs(1));
                continue;
            }

            self.run_round();
            thread::sleep(time::Duration::from_secs(10));
        }
    }

    // TODO: Add an upper limit to concurrency, deal with .unwrap()
    fn run_round(&mut self) -> Vec<(&Team, &Service, Result<CheckResult, CheckError>)> {
        thread::scope(|s| {
            let mut check_handles = vec![];

            for team in &self.config.teams {
                for service in &team.services {
                    let x = s.spawn(|| service.check.check());
                    check_handles.push((team, service, x));
                }
            }

            check_handles
                .into_iter()
                .map(|(t, s, h)| (t, s, h.join().unwrap()))
                .collect()
        })
    }
}
