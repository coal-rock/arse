use crate::engine::service::Service;

pub struct Team {
    pub name: String,
    pub services: Vec<Service>,
}
