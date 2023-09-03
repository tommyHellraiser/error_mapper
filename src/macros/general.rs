/// Macro that receives the **error to map**, and uses the From trait to convert it into a standard
/// **TheError** struct.
///
/// The original error message sent from the **origin crate** is mapped into the
/// self.error.error_content member.
#[macro_export]
macro_rules! map_to_new_error {
    ($error:expr) => {

        $crate::TheError {
            error: $crate::TheErrorType::from($error),
            file: file!().to_string(),
            location: format!("{}:{}", line!(), column!()),
            datestamp: chrono::Local::now().date_naive().format("%Y-%m-%d").to_string(),
            timestamp: chrono::Local::now().time().format("%H:%M:%S").to_string(),
        }
    }
}
