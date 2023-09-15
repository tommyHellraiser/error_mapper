
use crate::{SystemErrorCodes, TheErrorType};

impl From<actix_web::cookie::ParseError> for TheErrorType {
    fn from(value: actix_web::cookie::ParseError) -> Self {
        let error_type = match value {
            actix_web::cookie::ParseError::MissingPair => { SystemErrorCodes::ParseError }
            actix_web::cookie::ParseError::EmptyName => { SystemErrorCodes::ParseError }
            actix_web::cookie::ParseError::Utf8Error(_) => { SystemErrorCodes::ParseError }
            _ => { SystemErrorCodes::ParseError }
        };

        Self {
            error_type,
            error_content: value.to_string()
        }
    }
}

impl From<actix_web::cookie::time::Error> for TheErrorType {
    fn from(value: actix_web::cookie::time::Error) -> Self {
        let error_type = match value {
            actix_web::cookie::time::Error::ConversionRange(_) => { SystemErrorCodes::ConversionError }
            actix_web::cookie::time::Error::ComponentRange(_) => { SystemErrorCodes::OutOfRange }
            actix_web::cookie::time::Error::Format(_) => { SystemErrorCodes::FormatError }
            actix_web::cookie::time::Error::ParseFromDescription(_) => { SystemErrorCodes::ParseError }
            actix_web::cookie::time::Error::TryFromParsed(_) => { SystemErrorCodes::ParseError }
            actix_web::cookie::time::Error::InvalidFormatDescription(_) => { SystemErrorCodes::FormatError }
            actix_web::cookie::time::Error::DifferentVariant(_) => { SystemErrorCodes::ConversionError }
            actix_web::cookie::time::Error::InvalidVariant(_) => { SystemErrorCodes::ConversionError }
            _ => { SystemErrorCodes::Other }
        };

        Self {
            error_type,
            error_content: value.to_string()
        }
    }
}

impl From<actix_web::error::BlockingError> for TheErrorType {
    fn from(value: actix_web::error::BlockingError) -> Self {
        Self {
            error_type: SystemErrorCodes::LockError,
            error_content: value.to_string()
        }
    }
}

impl From<actix_web::error::ContentTypeError> for TheErrorType {
    fn from(value: actix_web::error::ContentTypeError) -> Self {
        let error_type = match value {
            actix_web::error::ContentTypeError::ParseError => { SystemErrorCodes::ParseError }
            actix_web::error::ContentTypeError::UnknownEncoding => { SystemErrorCodes::DecodeError }
            _ => { SystemErrorCodes::Other }
        };

        Self {
            error_type,
            error_content: value.to_string()
        }
    }
}

impl From<actix_web::error::DispatchError> for TheErrorType {
    fn from(value: actix_web::error::DispatchError) -> Self {
        Self {
            error_type: SystemErrorCodes::HttpRequestError,
            error_content: value.to_string()
        }
    }
}

impl From<actix_web::error::Error> for TheErrorType {
    fn from(value: actix_web::error::Error) -> Self {
        Self {
            error_type: SystemErrorCodes::HttpResponseError,
            error_content: value.to_string()
        }
    }
}

impl From<actix_web::error::HttpError> for TheErrorType {
    fn from(value: actix_web::error::HttpError) -> Self {
        Self {
            error_type: SystemErrorCodes::ConnectionError,
            error_content: value.to_string()
        }
    }
}

impl<T> From<actix_web::error::InternalError<T>> for TheErrorType {
    fn from(_: actix_web::error::InternalError<T>) -> Self {
        Self {
            error_type: SystemErrorCodes::ConnectionError,
            error_content: "Generic error from actix_web::error::InternalError".to_string()
        }
    }
}

impl From<actix_web::error::JsonPayloadError> for TheErrorType {
    fn from(value: actix_web::error::JsonPayloadError) -> Self {
        Self {
            error_type: SystemErrorCodes::ParseError,
            error_content: value.to_string()
        }
    }
}

impl From<actix_web::error::ParseError> for TheErrorType {
    fn from(value: actix_web::error::ParseError) -> Self {
        Self {
            error_type: SystemErrorCodes::ParseError,
            error_content: value.to_string()
        }
    }
}

impl From<actix_web::error::PathError> for TheErrorType {
    fn from(value: actix_web::error::PathError) -> Self {
        Self {
            error_type: SystemErrorCodes::HttpRequestError,
            error_content: value.to_string()
        }
    }
}

impl From<actix_web::error::PayloadError> for TheErrorType {
    fn from(value: actix_web::error::PayloadError) -> Self {
        Self {
            error_type: SystemErrorCodes::ParseError,
            error_content: value.to_string()
        }
    }
}

impl From<actix_web::error::QueryPayloadError> for TheErrorType {
    fn from(value: actix_web::error::QueryPayloadError) -> Self {
        Self {
            error_type: SystemErrorCodes::ParseError,
            error_content: value.to_string()
        }
    }
}

impl From<actix_web::error::ReadlinesError> for TheErrorType {
    fn from(value: actix_web::error::ReadlinesError) -> Self {
        Self {
            error_type: SystemErrorCodes::JsonError,
            error_content: value.to_string()
        }
    }
}

impl From<actix_web::error::UrlGenerationError> for TheErrorType {
    fn from(value: actix_web::error::UrlGenerationError) -> Self {
        Self {
            error_type: SystemErrorCodes::UrlError,
            error_content: value.to_string()
        }
    }
}

impl From<actix_web::error::UrlencodedError> for TheErrorType {
    fn from(value: actix_web::error::UrlencodedError) -> Self {
        Self {
            error_type: SystemErrorCodes::UrlError,
            error_content: value.to_string()
        }
    }
}

impl From<actix_web::http::Error> for TheErrorType {
    fn from(value: actix_web::http::Error) -> Self {
        Self {
            error_type: SystemErrorCodes::GenericError,
            error_content: value.to_string()
        }
    }
}

impl From<actix_web::http::header::ToStrError> for TheErrorType {
    fn from(value: actix_web::http::header::ToStrError) -> Self {
        Self {
            error_type: SystemErrorCodes::ConversionError,
            error_content: value.to_string()
        }
    }
}
