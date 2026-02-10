pub mod check;
mod random;

use crate::engine::checks::{
    check::{Check, CheckSchema},
    random::RandomCheck,
};

pub fn get_available_checks() -> Vec<CheckSchema> {
    vec![RandomCheck::get_check_schema()]
}
