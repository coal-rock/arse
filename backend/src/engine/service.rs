use crate::engine::checks::Check;

pub struct Service {
    name: String,
    weight: i32,
    check: Box<dyn Check>,
}
