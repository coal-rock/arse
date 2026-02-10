use crate::engine::checks::check::Check;

pub struct Service {
    pub name: String,
    pub weight: i32,
    pub check: Box<dyn Check + Send + Sync>,
}
