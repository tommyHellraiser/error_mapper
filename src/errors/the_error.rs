use std::fmt::{Display, Formatter};
use chrono::{NaiveDate, NaiveTime};
use crate::SystemErrorCodes;

/// Alias to a Result<T, TheError> type, with TheError being a struct that contains extra details
/// of the original error
pub type TheResult<T> = Result<T, TheError>;

/// Struct that contains the **error** itself mapped within a **TheErrorType** struct, with its **error
/// content** configured in the origin crate, the **file**, **location**, **datestamp** and
/// **timestamp** data of when the error was remapped using the map_to_new_error! macro.
#[derive(Debug, Default)]
pub struct TheError {
    pub error: TheErrorType,
    pub file: String,
    pub location: (u32, u32),
    pub datestamp: NaiveDate,
    pub timestamp: NaiveTime
}

/// Smaller error struct to contain the **mapped error type** as a **SystemErrorCodes** enum
/// and the **error content** from the origin error
#[derive(Debug, Default)]
pub struct TheErrorType {
    pub error_type: SystemErrorCodes,
    pub error_content: String,
}

impl TheError {

    /// **Returns** the SystemErrorCode associated with this error
    pub fn get_type(&self) -> &SystemErrorCodes {
        &self.error.error_type
    }

    /// **Returns** the error content mapped from the origin error
    pub fn get_content(&self) -> &String {
        &self.error.error_content
    }

    /// **Returns** the location String in file-line-column format
    pub fn get_location_info(&self) -> &(u32, u32) {
        &self.location
    }

    /// **Returns** the error's NaiveDate datestamp as a String
    pub fn get_datestamp(&self) -> &NaiveDate {
        &self.datestamp
    }

    /// **Returns** the error's NaiveTime timestamp as a String
    pub fn get_timestamp(&self) -> &NaiveTime {
        &self.timestamp
    }

    /// **Returns** the error's datetime stamp as a String
    pub fn get_datetime(&self) -> String {
        format!("{} {}", &self.datestamp, &self.timestamp).to_string()
    }

    /// **Description**: Adds the error type to the error
    pub fn with_type(mut self, error_type: SystemErrorCodes) -> Self {
        self.error.error_type = error_type;
        self
    }

    /// **Description**: Adds the error content to the error
    pub fn with_content(mut self, error_content: String) -> Self {
        self.error.error_content = error_content;
        self
    }

    /// **Description**: Adds the file data to the error
    pub fn with_file_data(mut self, file: String) -> Self {
        self.file = file;
        self
    }

    /// **Description**: Adds the location data to the error
    pub fn with_location_data(mut self, location: (u32, u32)) -> Self {
        self.location = location;
        self
    }

    /// **Description**: Adds the datestamp data to the error
    pub fn with_datestamp_data(mut self, datestamp: NaiveDate) -> Self {
        self.datestamp = datestamp;
        self
    }

    /// **Description**: Adds the timestamp data to the error
    pub fn with_timestamp_data(mut self, timestamp: NaiveTime) -> Self {
        self.timestamp = timestamp;
        self
    }
}

impl Display for TheError {
    /// Formatter function to **display** the error in a easy to user manner
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {

        write!(f, "{} T{}\t@ {} {}|{}\t\t\t\t{:?}: {}",
            self.datestamp,
            self.timestamp,
            self.file,
            self.location.0,
            self.location.1,
            self.error.error_type,
            self.error.error_content
        ).expect("Couldn't display message!!");

        Ok(())
    }
}