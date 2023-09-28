
use crate::{SystemErrorCodes, TheErrorType};

impl From<serde::de::value::Error> for TheErrorType {
    fn from(value: serde::de::value::Error) -> Self {
        Self {
            error_type: SystemErrorCodes::SerializationError,
            error_content: value.to_string()
        }
    }
}
