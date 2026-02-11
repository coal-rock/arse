pub mod check;
pub mod random;

use std::collections::HashMap;

use crate::engine::checks::{
    check::{Check, CheckFieldValidationError, CheckFieldValue, CheckSchema},
    random::RandomCheck,
};

pub fn get_available_checks() -> Vec<CheckSchema> {
    vec![RandomCheck::get_check_schema()]
}

pub fn configure_check(
    check_name: &str,
    check_config: HashMap<String, CheckFieldValue>,
) -> Result<Box<dyn Check + Send + Sync>, CheckFieldValidationError> {
    match check_name {
        "Random" => RandomCheck::configure(check_config)
            .map(|c| Box::new(c) as Box<dyn Check + Send + Sync>),
        _ => Result::Err(CheckFieldValidationError::InvalidCheckName),
    }
}
