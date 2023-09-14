use crate::{SystemErrorCodes, TheErrorType};

impl From<std::alloc::LayoutError> for TheErrorType {
    fn from(value: std::alloc::LayoutError) -> Self {
        Self {
            error_type: SystemErrorCodes::LayoutError,
            error_content: value.to_string()
        }
    }
}

impl From<std::array::TryFromSliceError> for TheErrorType {
    fn from(value: std::array::TryFromSliceError) -> Self {
        Self {
            error_type: SystemErrorCodes::ConversionError,
            error_content: value.to_string()
        }
    }
}

impl From<std::cell::BorrowError> for TheErrorType {
    fn from(value: std::cell::BorrowError) -> Self {
        Self {
            error_type: SystemErrorCodes::BorrowError,
            error_content: value.to_string()
        }
    }
}

impl From<std::cell::BorrowMutError> for TheErrorType {
    fn from(value: std::cell::BorrowMutError) -> Self {
        Self {
            error_type: SystemErrorCodes::BorrowError,
            error_content: value.to_string()
        }
    }
}

impl From<std::char::CharTryFromError> for TheErrorType {
    fn from(value: std::char::CharTryFromError) -> Self {
        Self {
            error_type: SystemErrorCodes::ConversionError,
            error_content: value.to_string()
        }
    }
}

impl From<std::char::DecodeUtf16Error> for TheErrorType {
    fn from(value: std::char::DecodeUtf16Error) -> Self {
        Self {
            error_type: SystemErrorCodes::DecodeError,
            error_content: value.to_string()
        }
    }
}

impl From<std::char::ParseCharError> for TheErrorType {
    fn from(value: std::char::ParseCharError) -> Self {
        Self {
            error_type: SystemErrorCodes::ConversionError,
            error_content: value.to_string()
        }
    }
}

impl From<std::char::TryFromCharError> for TheErrorType {
    fn from(value: std::char::TryFromCharError) -> Self {
        Self {
            error_type: SystemErrorCodes::ConversionError,
            error_content: value.to_string()
        }
    }
}

impl From<std::collections::TryReserveError> for TheErrorType {
    fn from(value: std::collections::TryReserveError) -> Self {
        Self {
            error_type: SystemErrorCodes::MemoryError,
            error_content: value.to_string()
        }
    }
}

impl From<std::env::JoinPathsError> for TheErrorType {
    fn from(value: std::env::JoinPathsError) -> Self {
        Self {
            error_type: SystemErrorCodes::JoinError,
            error_content: value.to_string()
        }
    }
}

impl From<std::env::VarError> for TheErrorType {
    fn from(value: std::env::VarError) -> Self {
        Self {
            error_type: SystemErrorCodes::EnvironmentError,
            error_content: value.to_string()
        }
    }
}

impl From<std::ffi::FromBytesWithNulError> for TheErrorType {
    fn from(value: std::ffi::FromBytesWithNulError) -> Self {
        Self {
            error_type: SystemErrorCodes::FormatError,
            error_content: value.to_string()
        }
    }
}

impl From<std::ffi::FromVecWithNulError> for TheErrorType {
    fn from(value: std::ffi::FromVecWithNulError) -> Self {
        Self {
            error_type: SystemErrorCodes::ConversionError,
            error_content: value.to_string()
        }
    }
}
impl From<std::ffi::IntoStringError> for TheErrorType {
    fn from(value: std::ffi::IntoStringError) -> Self {
        Self {
            error_type: SystemErrorCodes::ConversionError,
            error_content: value.to_string()
        }
    }
}

impl From<std::ffi::NulError> for TheErrorType {
    fn from(value: std::ffi::NulError) -> Self {
        Self {
            error_type: SystemErrorCodes::FormatError,
            error_content: value.to_string()
        }
    }
}

impl From<std::fmt::Error> for TheErrorType {
    fn from(value: std::fmt::Error) -> Self {
        Self {
            error_type: SystemErrorCodes::FormatError,
            error_content: value.to_string()
        }
    }
}

impl From<std::io::Error> for TheErrorType {
    fn from(value: std::io::Error) -> Self {
        Self {
            error_type: SystemErrorCodes::ReadWriteError,
            error_content: value.to_string()
        }
    }
}

impl From<std::io::ErrorKind> for TheErrorType {
    fn from(value: std::io::ErrorKind) -> Self {
        Self {
            error_type: SystemErrorCodes::Io,
            error_content: value.to_string()
        }
    }
}

impl<T> From<std::io::IntoInnerError<T>> for TheErrorType {
    fn from(value: std::io::IntoInnerError<T>) -> Self {
        Self {
            error_type: SystemErrorCodes::ConversionError,
            error_content: value.to_string()
        }
    }
}

impl From<std::net::AddrParseError> for TheErrorType {
    fn from(value: std::net::AddrParseError) -> Self {
        Self {
            error_type: SystemErrorCodes::Parse,
            error_content: value.to_string()
        }
    }
}

impl From<std::num::IntErrorKind> for TheErrorType {
    fn from(value: std::num::IntErrorKind) -> Self {
        let error_type;
        let error_content;
        match value {
            std::num::IntErrorKind::Empty => {
                error_type = SystemErrorCodes::Empty;
                error_content = "cannot parse integer from empty string".to_string();
            }
            std::num::IntErrorKind::InvalidDigit => {
                error_type = SystemErrorCodes::InvalidData;
                error_content = "invalid digit found in string".to_string();
            }
            std::num::IntErrorKind::PosOverflow => {
                error_type = SystemErrorCodes::Overflow;
                error_content = "number too large to fit in target type".to_string();
            }
            std::num::IntErrorKind::NegOverflow => {
                error_type = SystemErrorCodes::Overflow;
                error_content = "number too small to fit in target type".to_string();
            }
            std::num::IntErrorKind::Zero => {
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

impl From<std::num::ParseFloatError> for TheErrorType {
    fn from(value: std::num::ParseFloatError) -> Self {
        Self {
            error_type: SystemErrorCodes::Parse,
            error_content: value.to_string()
        }
    }
}

impl From<std::num::ParseIntError> for TheErrorType {
    fn from(value: std::num::ParseIntError) -> Self {
        Self {
            error_type: SystemErrorCodes::Parse,
            error_content: value.to_string()
        }
    }
}

impl From<std::num::TryFromIntError> for TheErrorType {
    fn from(value: std::num::TryFromIntError) -> Self {
        Self {
            error_type: SystemErrorCodes::ConversionError,
            error_content: value.to_string()
        }
    }
}

impl From<std::path::StripPrefixError> for TheErrorType {
    fn from(value: std::path::StripPrefixError) -> Self {
        Self {
            error_type: SystemErrorCodes::InvalidFilepath,
            error_content: value.to_string()
        }
    }
}

impl From<std::str::ParseBoolError> for TheErrorType {
    fn from(value: std::str::ParseBoolError) -> Self {
        Self {
            error_type: SystemErrorCodes::Parse,
            error_content: value.to_string()
        }
    }
}

impl From<std::str::Utf8Error> for TheErrorType {
    fn from(value: std::str::Utf8Error) -> Self {
        Self {
            error_type: SystemErrorCodes::ConversionError,
            error_content: value.to_string()
        }
    }
}

impl From<std::string::FromUtf16Error> for TheErrorType {
    fn from(value: std::string::FromUtf16Error) -> Self {
        Self {
            error_type: SystemErrorCodes::ConversionError,
            error_content: value.to_string()
        }
    }
}

impl From<std::string::FromUtf8Error> for TheErrorType {
    fn from(value: std::string::FromUtf8Error) -> Self {
        Self {
            error_type: SystemErrorCodes::ConversionError,
            error_content: value.to_string()
        }
    }
}

impl From<std::string::ParseError> for TheErrorType {
    fn from(value: std::string::ParseError) -> Self {
        Self {
            error_type: SystemErrorCodes::Parse,
            error_content: value.to_string()
        }
    }
}

impl<T> From<std::sync::PoisonError<T>> for TheErrorType {
    fn from(value: std::sync::PoisonError<T>) -> Self {
        Self {
            error_type: SystemErrorCodes::LockError,
            error_content: value.to_string()
        }
    }
}

impl<T> From<std::sync::TryLockError<T>> for TheErrorType {
    fn from(value: std::sync::TryLockError<T>) -> Self {
        Self {
            error_type: SystemErrorCodes::LockError,
            error_content: value.to_string()
        }
    }
}

impl From<std::sync::mpsc::RecvError> for TheErrorType {
    fn from(value: std::sync::mpsc::RecvError) -> Self {
        Self {
            error_type: SystemErrorCodes::ConnectionClosed,
            error_content: value.to_string()
        }
    }
}

impl From<std::sync::mpsc::RecvTimeoutError> for TheErrorType {
    fn from(value: std::sync::mpsc::RecvTimeoutError) -> Self {
        Self {
            error_type: SystemErrorCodes::TimedOut,
            error_content: value.to_string()
        }
    }
}

impl<T> From<std::sync::mpsc::SendError<T>> for TheErrorType {
    fn from(value: std::sync::mpsc::SendError<T>) -> Self {
        Self {
            error_type: SystemErrorCodes::ConnectionClosed,
            error_content: value.to_string()
        }
    }
}

impl From<std::sync::mpsc::TryRecvError> for TheErrorType {
    fn from(value: std::sync::mpsc::TryRecvError) -> Self {
        Self {
            error_type: SystemErrorCodes::ChannelError,
            error_content: value.to_string()
        }
    }
}

impl<T> From<std::sync::mpsc::TrySendError<T>> for TheErrorType {
    fn from(value: std::sync::mpsc::TrySendError<T>) -> Self {
        let error_type = match value {
            std::sync::mpsc::TrySendError::Full(_) => { SystemErrorCodes::ChannelError }
            std::sync::mpsc::TrySendError::Disconnected(_) => { SystemErrorCodes::ConnectionClosed }
        };

        Self {
            error_type,
            error_content: value.to_string()
        }
    }
}

impl From<std::thread::AccessError> for TheErrorType {
    fn from(value: std::thread::AccessError) -> Self {
        Self {
            error_type: SystemErrorCodes::ThreadError,
            error_content: value.to_string()
        }
    }
}

impl From<std::time::SystemTimeError> for TheErrorType {
    fn from(value: std::time::SystemTimeError) -> Self {
        Self {
            error_type: SystemErrorCodes::SystemTimeError,
            error_content: value.to_string()
        }
    }
}

impl From<std::time::TryFromFloatSecsError> for TheErrorType {
    fn from(value: std::time::TryFromFloatSecsError) -> Self {
        Self {
            error_type: SystemErrorCodes::ConversionError,
            error_content: value.to_string()
        }
    }
}
