
//  TODO: determine if it's possible to receive either a string message or a TheError struct inside a macro
#[derive(Default)]
#[allow(dead_code)]
pub enum LogLevel {
    FATAL,
    ERROR,
    WARNING,
    #[default]
    INFORMATION,
    DEBUG,
    TRACE
}