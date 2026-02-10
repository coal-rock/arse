use serde::Serialize;

use crate::engine::checks::check::{CheckFieldSchema, CheckSchema};

#[derive(Serialize, Clone)]
pub struct CheckSchemaResponse {
    pub name: String,
    pub description: String,
    pub fields: Vec<CheckFieldSchema>,
}

impl From<CheckSchema> for CheckSchemaResponse {
    fn from(value: CheckSchema) -> Self {
        CheckSchemaResponse {
            name: value.meta.name,
            description: value.meta.description,
            fields: value.fields,
        }
    }
}
