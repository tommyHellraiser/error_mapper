use chrono::{RoundingError, ParseMonthError, ParseWeekdayError, OutOfRangeError, ParseError};
use chrono::format::ParseErrorKind;
use crate::{SystemErrorCodes, TheErrorType};

impl From<RoundingError> for TheErrorType {
    fn from(value: RoundingError) -> Self {
        let error_content = value.to_string();
        let error_type = match value {
            RoundingError::DurationExceedsTimestamp => {
                SystemErrorCodes::BadDateFormat
            }
            RoundingError::DurationExceedsLimit => {
                SystemErrorCodes::BadDateFormat
            }
            RoundingError::TimestampExceedsLimit => {
                SystemErrorCodes::BadDateFormat
            }
        };
        Self {
            error_type,
            error_content
        }
    }
}

impl From<ParseMonthError> for TheErrorType {
    fn from(value: ParseMonthError) -> Self {
        Self{
            error_type: SystemErrorCodes::BadDateFormat,
            error_content: value.to_string()
        }
    }
}

impl From<ParseWeekdayError> for TheErrorType {
    fn from(value: ParseWeekdayError) -> Self {
        Self {
            error_type: SystemErrorCodes::BadDateFormat,
            error_content: value.to_string()
        }
    }
}

impl From<OutOfRangeError> for TheErrorType {
    fn from(value: OutOfRangeError) -> Self {
        Self {
            error_type: SystemErrorCodes::OutOfRangeDateValue,
            error_content: value.to_string()
        }
    }
}

impl From<ParseError> for TheErrorType {
    fn from(value: ParseError) -> Self {
        let error_content = value.to_string();
        let error_type = match value.kind() {
            ParseErrorKind::OutOfRange => {
                SystemErrorCodes::OutOfRangeDateValue
            }
            ParseErrorKind::Impossible => {
                SystemErrorCodes::InvalidDateValue
            }
            ParseErrorKind::NotEnough => {
                SystemErrorCodes::InvalidDateValue
            }
            ParseErrorKind::Invalid => {
                SystemErrorCodes::InvalidDateValue
            }
            ParseErrorKind::TooShort => {
                SystemErrorCodes::InvalidDateValue
            }
            ParseErrorKind::TooLong => {
                SystemErrorCodes::InvalidDateValue
            }
            ParseErrorKind::BadFormat => {
                SystemErrorCodes::BadDateFormat
            }
            ParseErrorKind::__Nonexhaustive => {
                SystemErrorCodes::BadDateFormat
            }
        };

        Self {
            error_type,
            error_content
        }
    }
}
