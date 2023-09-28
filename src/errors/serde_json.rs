
use crate::{SystemErrorCodes, TheErrorType};

impl From<serde_json::Error> for TheErrorType {
    fn from(value: serde_json::Error) -> Self {
        Self {
            error_type: SystemErrorCodes::JsonError,
            error_content: value.to_string()
        }
    }
}
