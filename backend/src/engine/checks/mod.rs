mod random;
use std::{collections::HashMap, net::IpAddr};

pub type CheckError = ();

pub enum CheckFieldValidationError {
    IncorrectNumberOfFields,
    UnexpectedField(String),
    InvalidFieldValue,
    MissingField(String),
}

pub enum CheckMessage {
    Text(String),
    Json(String),
}

pub enum CheckStatus {
    Up,
    Degreaded,
    Down,
}

pub struct CheckResult {
    status: CheckStatus,
    message: Option<CheckMessage>,
}

#[derive(Clone)]
pub enum CheckFieldValue {
    String(String),
    Username(String),
    Password(String),
    Number(i32),
    Percentage(f32),
    Float(f32),
    Duration(f32),
    Timeout(f32),
    Port(u16),
    IPAddrk(IpAddr),
}

impl CheckFieldValue {
    // TODO: Consider cleaning this up.
    // Macro magic might be able to make this a tad bit cleaner?
    // I really don't know, but this feels a little gross.
    pub fn as_string(self) -> Result<String, CheckFieldValidationError> {
        return match self {
            CheckFieldValue::String(value) => Ok(value.into()),
            CheckFieldValue::Username(value) => Ok(value.into()),
            CheckFieldValue::Password(value) => Ok(value.into()),
            _ => Result::Err(CheckFieldValidationError::InvalidFieldValue),
        };
    }

    pub fn as_i32(self) -> Result<i32, CheckFieldValidationError> {
        return match self {
            CheckFieldValue::Number(value) => Ok(value.into()),
            _ => Result::Err(CheckFieldValidationError::InvalidFieldValue),
        };
    }

    pub fn as_f32(self) -> Result<f32, CheckFieldValidationError> {
        return match self {
            CheckFieldValue::Percentage(value) => Ok(value.into()),
            CheckFieldValue::Float(value) => Ok(value.into()),
            CheckFieldValue::Duration(value) => Ok(value.into()),
            CheckFieldValue::Timeout(value) => Ok(value.into()),
            _ => Result::Err(CheckFieldValidationError::InvalidFieldValue),
        };
    }

    pub fn as_u16(self) -> Result<u16, CheckFieldValidationError> {
        return match self {
            CheckFieldValue::Port(value) => Ok(value.into()),
            _ => Result::Err(CheckFieldValidationError::InvalidFieldValue),
        };
    }

    pub fn as_ip(self) -> Result<IpAddr, CheckFieldValidationError> {
        return match self {
            CheckFieldValue::IPAddrk(value) => Ok(value.into()),
            _ => Result::Err(CheckFieldValidationError::InvalidFieldValue),
        };
    }
}

pub struct CheckFieldSchema {
    name: String,
    description: Option<String>,
    default: CheckFieldValue,
}

pub trait Check {
    fn get_fields_schema() -> Vec<CheckFieldSchema>
    where
        Self: Sized;

    fn check(self) -> Result<CheckResult, CheckError>;

    fn configure(
        check_fields: HashMap<String, CheckFieldValue>,
    ) -> Result<Self, CheckFieldValidationError>
    where
        Self: Sized;

    fn get_field(
        fields: &HashMap<String, CheckFieldValue>,
        field_name: &str,
    ) -> Result<CheckFieldValue, CheckFieldValidationError>
    where
        Self: Sized,
    {
        return match fields.get(field_name) {
            Some(field_value) => Ok(field_value.clone()),
            None => Result::Err(CheckFieldValidationError::MissingField(field_name.into())),
        };
    }
}
