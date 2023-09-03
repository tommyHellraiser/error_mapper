use std::fmt::{Display, Formatter};
use crate::SystemErrorCodes;

/// Alias to a Result<T, TheError> type, with TheError being a struct that contains extra details
/// of the original error
pub type TheResult<T> = Result<T, TheError>;

/// Struct that contains the **error** itself mapped within a **TheErrorType** struct, with its **error
/// content** configured in the origin crate, the **file**, **location**, **datestamp** and
/// **timestamp** data of when the error was remapped using the map_to_new_error! macro.
pub struct TheError {
    pub error: TheErrorType,
    pub file: String,
    pub location: String,
    pub datestamp: String,
    pub timestamp: String
}

impl Display for TheError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {

        write!(f, "\n{} {} @ {}::{} -> {:?}: {}",
               self.file,
               self.location,
               self.datestamp,
               self.timestamp,
               self.error.error_type,
               self.error.error_content
        ).expect("Couldn't display message!!");

        Ok(())
    }
}

/// Smaller error struct to contain the **mapped error type** as a **SystemErrorCodes** enum
/// and the **error content** from the origin error
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
    pub fn get_location_info(&self) -> &String {
        &self.location
    }

    /// **Returns** the error's NaiveDate datestamp as a String
    pub fn get_datestamp(&self) -> &String {
        &self.datestamp
    }

    /// **Returns** the error's NaiveTime timestamp as a String
    pub fn get_timestamp(&self) -> &String {
        &self.timestamp
    }

    /// **Returns** the error's datetime stamp as a String
    pub fn get_datetime(&self) -> String {
        format!("{} {}", &self.datestamp, &self.timestamp).to_string()
    }
}