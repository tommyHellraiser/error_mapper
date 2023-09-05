
//  TODO: determine if it's possible to receive either a string message or a TheError struct inside a macro
#[derive(Default)]
#[allow(dead_code)]
pub enum LogLevel {
    Fatal,
    Error,
    Warning,
    #[default]
    Information,
    Debug,
    Trace
}
