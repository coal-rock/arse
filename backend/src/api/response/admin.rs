use serde::Serialize;

use crate::engine::checks::check::{CheckFieldSchema, CheckSchema};

#[derive(Serialize, Clone, ts_rs::TS)]
#[ts(export)]
pub struct CheckSchemaResponse {
    pub name: String,
    pub description: String,
    pub fields: Vec<CheckFieldSchema>,
}

impl From<&CheckSchema> for CheckSchemaResponse {
    fn from(value: &CheckSchema) -> Self {
        CheckSchemaResponse {
            name: value.meta.name.clone(),
            description: value.meta.description.clone(),
            fields: value.fields.clone(),
        }
    }
}

#[derive(Serialize, Clone)]
pub struct EngineStatusResponse {
    pub running: bool,
}
