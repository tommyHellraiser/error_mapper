use chrono::{RoundingError, ParseMonthError, ParseWeekdayError, OutOfRangeError, ParseError};
use chrono::format::ParseErrorKind;
use crate::{SystemErrorCodes};

impl From<RoundingError> for SystemErrorCodes {
    fn from(value: RoundingError) -> Self {
        match value {
            RoundingError::DurationExceedsTimestamp => {SystemErrorCodes::BadDateFormat }
            RoundingError::DurationExceedsLimit => {SystemErrorCodes::BadDateFormat }
            RoundingError::TimestampExceedsLimit => {SystemErrorCodes::BadDateFormat }
        }
    }
}

impl From<ParseMonthError> for SystemErrorCodes {
    fn from(_: ParseMonthError) -> Self {
        SystemErrorCodes::BadDateFormat
    }
}

impl From<ParseWeekdayError> for SystemErrorCodes {
    fn from(_: ParseWeekdayError) -> Self {
        SystemErrorCodes::BadDateFormat
    }
}

impl From<OutOfRangeError> for SystemErrorCodes {
    fn from(_: OutOfRangeError) -> Self {
        SystemErrorCodes::OutOfRangeDateValue
    }
}

impl From<ParseError> for SystemErrorCodes {
    fn from(value: ParseError) -> Self {
        match value.kind() {
            ParseErrorKind::OutOfRange => {SystemErrorCodes::OutOfRangeDateValue}
            ParseErrorKind::Impossible => {SystemErrorCodes::InvalidDateValue}
            ParseErrorKind::NotEnough => {SystemErrorCodes::InvalidDateValue}
            ParseErrorKind::Invalid => {SystemErrorCodes::InvalidDateValue}
            ParseErrorKind::TooShort => {SystemErrorCodes::InvalidDateValue}
            ParseErrorKind::TooLong => {SystemErrorCodes::InvalidDateValue}
            ParseErrorKind::BadFormat => {SystemErrorCodes::BadDateFormat}
            ParseErrorKind::__Nonexhaustive => {SystemErrorCodes::BadDateFormat}
        }
    }
}
