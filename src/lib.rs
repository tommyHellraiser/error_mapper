extern crate core;

mod macros;
mod errors;
mod logger;

pub use errors::the_error::{TheResult, TheError, TheErrorType};
pub use errors::system_error_codes::SystemErrorCodes;

