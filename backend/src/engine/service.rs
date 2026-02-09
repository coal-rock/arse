use crate::engine::checks::check::Check;

pub struct Service {
    name: String,
    weight: i32,
    check: Box<dyn Check>,
}
