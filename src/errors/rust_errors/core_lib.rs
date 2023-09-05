use crate::{SystemErrorCodes, TheErrorType};

impl From<core::alloc::LayoutError> for TheErrorType {
    fn from(value: core::alloc::LayoutError) -> Self {
        Self {
            error_type: SystemErrorCodes::LayoutError,
            error_content: value.to_string()
        }
    }
}

impl From<core::array::TryFromSliceError> for TheErrorType {
    fn from(value: core::array::TryFromSliceError) -> Self {
        Self {
            error_type: SystemErrorCodes::ConversionError,
            error_content: value.to_string()
        }
    }
}

impl From<core::cell::BorrowError> for TheErrorType {
    fn from(value: core::cell::BorrowError) -> Self {
        Self {
            error_type: SystemErrorCodes::BorrowError,
            error_content: value.to_string()
        }
    }
}

impl From<core::cell::BorrowMutError> for TheErrorType {
    fn from(value: core::cell::BorrowMutError) -> Self {
        Self {
            error_type: SystemErrorCodes::BorrowError,
            error_content: value.to_string()
        }
    }
}

impl From<core::char::CharTryFromError> for TheErrorType {
    fn from(value: core::char::CharTryFromError) -> Self {
        Self {
            error_type: SystemErrorCodes::ConversionError,
            error_content: value.to_string()
        }
    }
}

impl From<core::char::DecodeUtf16Error> for TheErrorType {
    fn from(value: core::char::DecodeUtf16Error) -> Self {
        Self {
            error_type: SystemErrorCodes::DecodeError,
            error_content: value.to_string()
        }
    }
}

impl From<core::char::ParseCharError> for TheErrorType {
    fn from(value: core::char::ParseCharError) -> Self {
        Self {
            error_type: SystemErrorCodes::ConversionError,
            error_content: value.to_string()
        }
    }
}

impl From<core::char::TryFromCharError> for TheErrorType {
    fn from(value: core::char::TryFromCharError) -> Self {
        Self {
            error_type: SystemErrorCodes::ConversionError,
            error_content: value.to_string()
        }
    }
}

impl From<core::ffi::FromBytesUntilNulError> for TheErrorType {
    fn from(value: core::ffi::FromBytesUntilNulError) -> Self {
        Self {
            error_type: SystemErrorCodes::ConversionError,
            error_content: value.to_string()
        }
    }
}

impl From<core::ffi::FromBytesWithNulError> for TheErrorType {
    fn from(value: core::ffi::FromBytesWithNulError) -> Self {
        Self {
            error_type: SystemErrorCodes::ConversionError,
            error_content: value.to_string()
        }
    }
}

impl From<core::fmt::Error> for TheErrorType {
    fn from(value: core::fmt::Error) -> Self {
        Self {
            error_type: SystemErrorCodes::FormatError,
            error_content: value.to_string()
        }
    }
}

impl From<core::num::IntErrorKind> for TheErrorType {
    fn from(value: core::num::IntErrorKind) -> Self {
        let error_type;
        let error_content;
        match value {
            core::num::IntErrorKind::Empty => {
                error_type = SystemErrorCodes::Empty;
                error_content = "cannot parse integer from empty string".to_string();
            }
            core::num::IntErrorKind::InvalidDigit => {
                error_type = SystemErrorCodes::InvalidData;
                error_content = "invalid digit found in string".to_string();
            }
            core::num::IntErrorKind::PosOverflow => {
                error_type = SystemErrorCodes::Overflow;
                error_content = "number too large to fit in target type".to_string();
            }
            core::num::IntErrorKind::NegOverflow => {
                error_type = SystemErrorCodes::Overflow;
                error_content = "number too small to fit in target type".to_string();
            }
            core::num::IntErrorKind::Zero => {
                error_type = SystemErrorCodes::Zero;
                error_content = "number would be zero for non-zero type".to_string();
            }
            _ => {
                error_type = SystemErrorCodes::GenericError;
                error_content = "generic type error".to_string();
            }
        }
        Self {
            error_type,
            error_content
        }
    }
}

impl From<core::num::ParseFloatError> for TheErrorType {
    fn from(value: core::num::ParseFloatError) -> Self {
        Self {
            error_type: SystemErrorCodes::ConversionError,
            error_content: value.to_string()
        }
    }
}

impl From<core::num::ParseIntError> for TheErrorType {
    fn from(value: core::num::ParseIntError) -> Self {
        Self {
            error_type: SystemErrorCodes::ConversionError,
            error_content: value.to_string()
        }
    }
}

impl From<core::num::TryFromIntError> for TheErrorType {
    fn from(value: core::num::TryFromIntError) -> Self {
        Self {
            error_type: SystemErrorCodes::ConversionError,
            error_content: value.to_string()
        }
    }
}

impl From<core::str::ParseBoolError> for TheErrorType {
    fn from(value: core::str::ParseBoolError) -> Self {
        Self {
            error_type: SystemErrorCodes::ConversionError,
            error_content: value.to_string()
        }
    }
}

impl From<core::str::Utf8Error> for TheErrorType {
    fn from(value: core::str::Utf8Error) -> Self {
        Self {
            error_type: SystemErrorCodes::ConversionError,
            error_content: value.to_string()
        }
    }
}

impl From<core::time::TryFromFloatSecsError> for TheErrorType {
    fn from(value: core::time::TryFromFloatSecsError) -> Self {
        Self {
            error_type: SystemErrorCodes::ConversionError,
            error_content: value.to_string()
        }
    }
}
