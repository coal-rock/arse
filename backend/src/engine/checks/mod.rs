pub mod check;
mod random;

use crate::engine::checks::check::{Check, CheckFieldSchema};
use std::collections::HashMap;

pub fn get_available_checks() -> HashMap<String, Vec<CheckFieldSchema>> {
    vec![("Random".into(), random::RandomCheck::get_fields_schema())]
        .iter()
        .cloned()
        .collect()
}
