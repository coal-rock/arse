use rand::random_bool;
use std::collections::HashMap;

use crate::engine::checks::{
    Check, CheckError, CheckFieldSchema, CheckFieldValidationError, CheckFieldValue, CheckResult,
    CheckStatus,
};

struct RandomCheck {
    likelihood: f32,
}

impl Check for RandomCheck {
    fn get_fields_schema() -> Vec<CheckFieldSchema> {
        return vec![CheckFieldSchema {
            name: "Likelihood".into(),
            description: Some("Percent of the time that check should succeed.".into()),
            default: CheckFieldValue::Percentage(50.0f32),
        }];
    }

    fn configure(
        check_fields: HashMap<String, CheckFieldValue>,
    ) -> Result<Self, CheckFieldValidationError> {
        Ok(Self {
            likelihood: RandomCheck::get_field(&check_fields, "Likelihood")?.as_f32()?,
        })
    }

    fn check(self) -> Result<CheckResult, CheckError> {
        let likelihood: f32 = self.likelihood / 100.0;
        let likelihood: f32 = likelihood.clamp(0.0, 1.0);

        Ok(CheckResult {
            status: match random_bool(likelihood as f64) {
                true => CheckStatus::Up,
                false => CheckStatus::Down,
            },
            message: None,
        })
    }
}
