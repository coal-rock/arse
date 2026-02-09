use std::net::IpAddr;

pub type CheckError = ();

pub enum CheckMessage {
    Text(String),
    Json(String),
}

pub enum CheckStatus {
    Good,
    Degreaded,
    Down,
}

pub struct CheckResult {
    status: CheckStatus,
    message: Option<CheckMessage>,
}

pub enum CheckFieldType {
    String,
    Number,
    Username,
    Password,
    Duration,
    Timeout,
    IPAddr,
    Port,
}

pub enum CheckFieldValue {
    String(String),
    Number(i32),
    Float(f32),
    Username(String),
    Password(String),
    Duration(f32),
    Timeout(f32),
    IPAddrk(IpAddr),
    Port(u16),
}

pub struct CheckFieldSchema {
    name: String,
    description: Option<String>,
    r#type: CheckFieldType,
}

pub trait Check {
    fn get_fields(self) -> Vec<CheckFieldSchema>;
    fn check(self) -> Result<CheckError, CheckResult>;

    fn configure(check_fields: (String, CheckFieldValue)) -> Self
    where
        Self: Sized;
}
