# error_mapper crate

Small crate to handle errors and results more easily. Simply include it in the dependencies section using the latest
available version and enable the features that will include mapped errors from different crates.

## Error mapping from the following crates is currently supported:
 - std Rust errors (rustc v1.72.0 on Windows OS), standard and core libraries
 - mysql_async (v0.32.2)  -> https://crates.io/crates/mysql_async/0.32.2
 - chrono (v0.4.26) -> https://crates.io/crates/chrono/0.4.26
 - tokio (v1.29.1) -> https://crates.io/crates/tokio/1.29.1
 - actix-web (v4.4.0) -> https://crates.io/crates/actix-web/4.4.0
 - reqwest (v0.11.20) -> https://crates.io/crates/reqwest/0.11.20
 - serde (v1.0.188) -> https://crates.io/crates/serde/1.0.188
 - serde_json (v1.0.107) -> https://crates.io/crates/serde_json/1.0.107

## Features

The features for this crate are determined by the external crates supported. If you want to use the mapping for tokio
for example, add the feature `"tokio"`, the same way with `chrono`, or any other crate you want to map.
The only error support that'll be loaded by default and cannot be disabled is the support for std Rust errors.

To use only the functions and the TheResult type, add to dependencies with no features, and to include mapping for every 
crate available, use the `full` feature.

## Main data types

The result enum is a customized `TheResult` type, which is an alias for `Result<T, TheError>`, where TheError is a struct type containing the following members:

```Rust
pub struct TheError {
    pub error: TheErrorType,
    pub file: String,
    pub location: (u32, u32),
    pub datestamp: NaiveDate,
    pub timestamp: NaiveTime
}
```

Where the error member of type `TheErrorType` is a struct containing:

```Rust
pub struct TheErrorType {
    pub error_type: SystemErrorCodes,
    pub error_content: String,
}
```

`SystemErrorCodes` is an enum type that contains all the possible error types with a good degree of precision. It's 
bound to keep growing as more crates are added to `error_mapper`.

This crate implements the `std::fmt::Display` trait for all its error data types. The output result of the fully
displayed error will be similar to the following example:

``GenericError: This was the created error!!``

First the file where the error occurred is displayed, followed by the number of line and column. The next element 
displayed is the date and time of the error for the local timezone (to log the date and time Utc::now() is used).
To the right of the arrow, the error type will be displayed, and following the colon, the error message. This message 
comes directly from the crate, it's remapped here but its contents are not modified, meaning that if you got a 
connection error with, let's say mysql_async, the error message returned from that crate will be redirected to your 
output. 

## Usage Example

Using error_mapper is very simple. To demonstrate it, we'll use an example of an error connection to a MySQL database, 
using the mysql_async crate:

```Rust
async fn example_fn() -> TheResult<Conn> {

    //  This macro will map any of the supported crates' error to The Error
    let pool = match Pool::from_url(EnvironmentConfig::instance().get_db_url().await) {
        Ok(pool) => pool,
        Err(e) => {
            return Err(map_to_new_error!(e))
        }
    };

    //  This second macro will enable you to create a new error with a custom message and 
    // an error type, in case you don't have any error to map or is not convenient to do so
    match pool.get_conn().await {
        Ok(conn) => Ok(conn),
        Err(e) => {
            Err(
                create_new_error!(
                    SystemErrorCodes::DbConnectionError, 
                    "Failed to connect to database"
                )
            );
        }
    }
    
    //  Alternatively, this macro allows another variant that only receives an error message.
    // The error type in this case, will be defined as SystemErrorCodes::GenericError
    match pool.get_conn().await {
        Ok(conn) => Ok(conn),
        Err(e) => {
            Err(create_new_error!("Failed to connect to database"));
        }
    }
}
```

The pool variable represents the connection pool returned from trying to contact the database, and will return a 
`mysql_async::Error` error type if it fails.

Since we don't want to deal with this specific error type, we'll use the 
`map_to_new_error!` macro to deal with a generalized and simpler type of error, that actually retains all the original info and adds 
some new important and useful data such as the date, time and file location of the error.

The calling function of example_fn() will receive the error as a `TheError` type, enclosed in a TheResult type. And since 
the type `TheResult` is an alias for the enum `core::Result` built into Rust, you can propagate the error to control 
the execution flow, and handle it elsewhere in case it fails.

The previous example instructions are now contained inside the async function get_conn(), then you can call it from our 
example_fn() and propagate the error if it fails, but continue execution normally if it doesn't:

```Rust
async fn example_fn() -> TheResult<Conn> {

    let conn = get_conn().await?;
    
    Ok(conn)
}
```
