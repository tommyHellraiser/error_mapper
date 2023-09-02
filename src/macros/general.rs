
#[macro_export]
macro_rules! new_error {
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
