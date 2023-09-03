use chrono::{RoundingError, ParseMonthError, ParseWeekdayError, OutOfRangeError, ParseError};
use chrono::format::ParseErrorKind;
use crate::{SystemErrorCodes, TheErrorType};

impl From<RoundingError> for TheErrorType {
    fn from(value: RoundingError) -> Self {
        let error_type;
        let error_content = value.to_string();
        match value {
            RoundingError::DurationExceedsTimestamp => {
                error_type = SystemErrorCodes::BadDateFormat
            }
            RoundingError::DurationExceedsLimit => {
                error_type = SystemErrorCodes::BadDateFormat
            }
            RoundingError::TimestampExceedsLimit => {
                error_type = SystemErrorCodes::BadDateFormat
            }
        }
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
        let error_type;
        let error_content = value.to_string();
        match value.kind() {
            ParseErrorKind::OutOfRange => {
                error_type = SystemErrorCodes::OutOfRangeDateValue
            }
            ParseErrorKind::Impossible => {
                error_type = SystemErrorCodes::InvalidDateValue
            }
            ParseErrorKind::NotEnough => {
                error_type = SystemErrorCodes::InvalidDateValue
            }
            ParseErrorKind::Invalid => {
                error_type = SystemErrorCodes::InvalidDateValue
            }
            ParseErrorKind::TooShort => {
                error_type = SystemErrorCodes::InvalidDateValue
            }
            ParseErrorKind::TooLong => {
                error_type = SystemErrorCodes::InvalidDateValue
            }
            ParseErrorKind::BadFormat => {
                error_type = SystemErrorCodes::BadDateFormat
            }
            ParseErrorKind::__Nonexhaustive => {
                error_type = SystemErrorCodes::BadDateFormat
            }
        }

        Self {
            error_type,
            error_content
        }
    }
}
