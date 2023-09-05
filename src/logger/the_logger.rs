#![allow(dead_code)]
use lazy_static::lazy_static;
use tokio::sync::RwLock;

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

lazy_static!(
    static ref THE_LOGGER: TheLogger = TheLogger::new();
);

pub struct TheLogger {
    config: RwLock<TheLoggerConfig>
}

#[derive(Default)]
struct TheLoggerConfig {

}

impl TheLogger {
    pub fn new() -> Self {
        Self {
            config: RwLock::new(TheLoggerConfig::default())
        }
    }

    pub fn hide_date() {

    }

    pub fn hide_years() {

    }

    pub fn hide_months() {

    }

    pub fn hide_days() {

    }

    pub fn hide_timestamp() {

    }

    pub fn timestamp_format() {

    }

    pub fn hide_hours() {

    }

    pub fn hide_minutes() {

    }

    pub fn hide_secs() {

    }

    pub fn hide_millisecs() {

    }

    pub fn hide_level() {

    }

    pub fn level_with_square_brackets() {

    }

    pub fn hide_error_line() {

    }

    pub fn hide_error_column() {

    }
}


/*
YYYY-MM-DD HH:MM:SS.mm LEVEL \t @ FILE LINE:COL -> Error itself
 */