use serde::{Deserialize, Serialize};
use std::{collections::HashMap, net::IpAddr};

pub type CheckError = ();

#[derive(Debug)]
pub enum CheckFieldValidationError {
    IncorrectNumberOfFields,
    UnexpectedField(String),
    InvalidFieldValue,
    MissingField(String),
    InvalidCheckName,
}

#[derive(Clone)]
pub enum CheckMessage {
    Text(String),
    Json(String),
}

#[derive(Clone)]
pub enum CheckStatus {
    Up,
    Degreaded,
    Down,
}

#[derive(Clone)]
pub struct CheckResult {
    pub status: CheckStatus,
    pub message: Option<CheckMessage>,
}

#[derive(Clone, Serialize, Deserialize)]
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

// TODO: Consider cleaning this up.
// Macro magic might be able to make this a tad bit cleaner?
// I really don't know, but this feels a little gross.
pub trait ExtractCheckFieldValue: Sized {
    fn extract(value: CheckFieldValue) -> Result<Self, CheckFieldValidationError>;
}

impl ExtractCheckFieldValue for String {
    fn extract(value: CheckFieldValue) -> Result<Self, CheckFieldValidationError> {
        return match value {
            CheckFieldValue::String(value) => Ok(value),
            CheckFieldValue::Username(value) => Ok(value.into()),
            CheckFieldValue::Password(value) => Ok(value.into()),
            _ => Result::Err(CheckFieldValidationError::InvalidFieldValue),
        };
    }
}

impl ExtractCheckFieldValue for i32 {
    fn extract(value: CheckFieldValue) -> Result<Self, CheckFieldValidationError> {
        return match value {
            CheckFieldValue::Number(value) => Ok(value.into()),
            _ => Result::Err(CheckFieldValidationError::InvalidFieldValue),
        };
    }
}

impl ExtractCheckFieldValue for f32 {
    fn extract(value: CheckFieldValue) -> Result<Self, CheckFieldValidationError> {
        return match value {
            CheckFieldValue::Percentage(value) => Ok(value),
            CheckFieldValue::Float(value) => Ok(value),
            CheckFieldValue::Duration(value) => Ok(value),
            CheckFieldValue::Timeout(value) => Ok(value),
            _ => Result::Err(CheckFieldValidationError::InvalidFieldValue),
        };
    }
}

impl ExtractCheckFieldValue for u16 {
    fn extract(value: CheckFieldValue) -> Result<Self, CheckFieldValidationError> {
        return match value {
            CheckFieldValue::Port(value) => Ok(value.into()),
            _ => Result::Err(CheckFieldValidationError::InvalidFieldValue),
        };
    }
}

impl ExtractCheckFieldValue for IpAddr {
    fn extract(value: CheckFieldValue) -> Result<Self, CheckFieldValidationError> {
        return match value {
            CheckFieldValue::IPAddrk(value) => Ok(value.into()),
            _ => Result::Err(CheckFieldValidationError::InvalidFieldValue),
        };
    }
}

#[derive(Serialize, Clone)]
pub struct CheckMeta {
    pub name: String,
    pub description: String,
}

#[derive(Clone, Serialize)]
pub struct CheckFieldSchema {
    pub name: String,
    pub description: Option<String>,
    pub default: CheckFieldValue,
}

#[derive(Serialize, Clone)]
pub struct CheckSchema {
    pub meta: CheckMeta,
    pub fields: Vec<CheckFieldSchema>,
}

pub trait Check {
    fn get_check_meta() -> CheckMeta
    where
        Self: Sized;

    fn get_fields_schema() -> Vec<CheckFieldSchema>
    where
        Self: Sized;

    fn get_check_schema() -> CheckSchema
    where
        Self: Sized,
    {
        return CheckSchema {
            meta: Self::get_check_meta(),
            fields: Self::get_fields_schema(),
        };
    }

    fn check(&self) -> Result<CheckResult, CheckError>;

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

#[macro_export]
macro_rules! check {
    (
    Ident: $check_ident:ident,
    Name: $name:literal,
    Description: $description:literal,
    Fields: [$( $field_ident:ident:
                    Name: $field_name:literal,
                    Description: $field_desc:literal,
                    Default: $field_value:expr,
                    Type: $field_type:ty,
            )*]

    $check:item
    ) => {
        use crate::engine::checks::check::{
            Check, CheckError, CheckFieldSchema, CheckFieldValidationError, CheckFieldValue, CheckMeta,
            CheckResult, CheckStatus, ExtractCheckFieldValue,
        };

        use std::collections::HashMap;

        #[allow(dead_code)]
        pub struct $check_ident {
            $($field_ident: $field_type,)*
        }

        impl Check for $check_ident {

            #[allow(dead_code)]
            fn get_check_meta() -> CheckMeta {
                CheckMeta {
                    name: $name.into(),
                    description: $description.into(),
                }
            }

            #[allow(dead_code)]
            fn get_fields_schema() -> Vec<CheckFieldSchema> {
                return vec![
                    $(
                        CheckFieldSchema {
                            name: $field_name.into(),
                            description: Some($field_desc.into()),
                            default: $field_value,
                        },
                    )*
                ];
            }

            #[allow(dead_code)]
            fn configure(
                check_fields: HashMap<String, CheckFieldValue>,
            ) -> Result<Self, CheckFieldValidationError> {
                Ok(Self {
                    $($field_ident: <$field_type as ExtractCheckFieldValue>::extract($check_ident::get_field(&check_fields, $field_name)?)?,)*
                })
            }

            $check
        }

        impl $check_ident {
            #[allow(dead_code)]
            pub fn new(
                $($field_ident: $field_type,)*
            ) -> Self {

                Self {
                    $($field_ident: $field_ident,)*
                }
            }
        }
    };
}
