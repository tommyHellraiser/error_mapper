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
    pub file: Option<String>,
    pub location: Option<(u32, u32)>,
    pub datestamp: Option<NaiveDate>,
    pub timestamp: Option<NaiveTime>
}

/// Smaller error struct to contain the **mapped error type** as a **SystemErrorCodes** enum
/// and the **error content** from the origin error
#[derive(Debug, Default)]
pub struct TheErrorType {
    pub error_type: SystemErrorCodes,
    pub error_content: String,
}

impl TheError {
    /// **Description**: Creates a new error of TheError type, receiving as parameters the error
    /// type and content, and capturing the file, location, datestamp and timestamp info
    pub fn new(
        error_type: SystemErrorCodes,
        error_content: String
    ) -> Self {
        Self {
            error: TheErrorType {
                error_type,
                error_content
            },
            file: None,
            location: None,
            datestamp: None,
            timestamp: None
        }
    }

    /// **Returns** the SystemErrorCode associated with this error
    pub fn get_type(&self) -> &SystemErrorCodes {
        &self.error.error_type
    }

    /// **Returns** the error content mapped from the origin error
    pub fn get_content(&self) -> &String {
        &self.error.error_content
    }

    /// **Returns** the location String in file-line-column format
    pub fn get_location_info(&self) -> &Option<(u32, u32)> {
        &self.location
    }

    /// **Returns** the error's NaiveDate datestamp as a String
    pub fn get_datestamp(&self) -> &Option<NaiveDate> {
        &self.datestamp
    }

    /// **Returns** the error's NaiveTime timestamp as a String
    pub fn get_timestamp(&self) -> &Option<NaiveTime> {
        &self.timestamp
    }

    /// **Returns** the error's datetime stamp as a String
    pub fn get_datetime(&self) -> Option<(String, String)> {
        // format!("{} {}", &self.datestamp, &self.timestamp).to_string()
        let date = if let Some(datestamp) = self.datestamp {
            datestamp.to_string()
        } else {
            "".to_string()
        };
        let time = if let Some(timestamp) = self.timestamp {
            timestamp.to_string()
        } else {
            "".to_string()
        };

        Some((date, time))
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
        self.file = Some(file);
        self
    }

    /// **Description**: Adds the location data to the error
    pub fn with_location_data(mut self, location: (u32, u32)) -> Self {
        self.location = Some(location);
        self
    }

    /// **Description**: Adds the datestamp data to the error
    pub fn with_datestamp_data(mut self, datestamp: NaiveDate) -> Self {
        self.datestamp = Some(datestamp);
        self
    }

    /// **Description**: Adds the timestamp data to the error
    pub fn with_timestamp_data(mut self, timestamp: NaiveTime) -> Self {
        self.timestamp = Some(timestamp);
        self
    }

    /// **Description**: Allows the user to add the error type after creation
    pub fn add_error_type(&mut self, error_type: SystemErrorCodes) {
        self.error.error_type = error_type;
    }

    /// **Description**: Allows the user to add the error content after creation
    pub fn add_error_content(&mut self, content: String) {
        self.error.error_content = content;
    }

    /// **Description**: Allows the user to add the file data after creation
    pub fn add_file_data(&mut self, file: String) {
        self.file = Some(file);
    }

    /// **Description**: Allows the user to add the location data after creation
    pub fn add_location_data(&mut self, location: (u32, u32)) {
        self.location = Some(location);
    }

    /// **Description**: Allows the user to add the datestamp data after creation
    pub fn add_datestamp_data(&mut self, datestamp: NaiveDate) {
        self.datestamp = Some(datestamp);
    }

    /// **Description**: Allows the user to add the timestamp data after creation
    pub fn add_timestamp_data(&mut self, timestamp: NaiveTime) {
        self.timestamp = Some(timestamp);
    }
}

impl Display for TheError {
    /// Formatter function to **display** the error in a simple manner.
    ///
    /// No overhead, just the error type and content. If the user needs more information and was
    /// previously stored, it'll be inside TheError struct.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {

        write!(f, "{}: {}", self.error.error_type, self.error.error_content)
            .expect("Couldn't display message!!");

        Ok(())
    }
}

impl From<TheError> for TheErrorType {
    fn from(value: TheError) -> Self {
        Self {
            error_type: value.error.error_type,
            error_content: value.error.error_content
        }
    }
}
