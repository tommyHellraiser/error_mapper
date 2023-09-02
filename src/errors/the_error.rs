use std::fmt::{Display, Formatter};
use crate::SystemErrorCodes;

pub type TheResult<T> = Result<T, TheError>;

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

pub struct TheErrorType {
    pub error_type: SystemErrorCodes,
    pub error_content: String,
}
