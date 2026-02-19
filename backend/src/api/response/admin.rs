use serde::Serialize;

use crate::db::users::UserDB;
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

#[derive(Serialize, Clone, ts_rs::TS)]
#[ts(export)]
pub struct ListUsersResponse {
    pub users: Vec<UsersResponse>,
}

impl From<Vec<UserDB>> for ListUsersResponse {
    fn from(value: Vec<UserDB>) -> Self {
        ListUsersResponse {
            users: value.into_iter().map(|u| u.into()).collect(),
        }
    }
}

#[derive(Serialize, Clone, ts_rs::TS)]
#[ts(export)]
pub struct UsersResponse {
    username: String,
    enabled: bool,
    created: i64,
}

impl From<UserDB> for UsersResponse {
    fn from(value: UserDB) -> Self {
        UsersResponse {
            username: value.username,
            enabled: value.enabled,
            created: value.created.and_utc().timestamp(),
        }
    }
}
