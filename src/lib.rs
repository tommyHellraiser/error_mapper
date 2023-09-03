mod macros;
mod errors;

pub use errors::the_error::{TheResult, TheError, TheErrorType};
pub use errors::system_error_codes::SystemErrorCodes;

//  TODO. add support for tokio errors
//  TODO: Check if I need to add support for std::io errors
//  TODO: add some sort of logger to log any TheError error into a .log file with a macro, and add a message if the user wants to