
use crate::{SystemErrorCodes, TheErrorType};

impl From<reqwest::Error> for TheErrorType {
    fn from(value: reqwest::Error) -> Self {
        Self {
            error_type: SystemErrorCodes::HttpRequestError,
            error_content: value.to_string()
        }
    }
}
