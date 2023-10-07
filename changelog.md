# v0.3.8 (2023-10-07)
### Fixed
- Fixed error in `map_to_new_error!` macro
### Added
- Generic ``Error`` in `SystemErrorCodes`

# v0.3.7 (2023-10-07)
### Added
- Added a new macro `create_new_error!` to only send the error type and content
- Added functions to append error details after its creation
### Modified
- Simplified the Display output to avoid too much information displayed
- Made every parameter of `TheError` an `Option` except for the error type and content

# v0.3.6 (2023-09-30)
- Added `new()` function for `TheError`
- Added `From` implementation for `TheErrorType`
- Restructured the `Display` output format for `TheError`

# v0.3.5 (2023-06-28)
- Added construction functions to TheError

# v0.3.4 (2023-09-28)
- Added support for crates serde and serde_json

# v0.3.3 (2023-09-28)
- Added support for crate reqwest v0.11.20

# v0.3.2 (2023-09-15)
- Formatted the error display output
- Support for actix-web errors is now available

# v0.3.1 (2023-09-14)
- TheError type now implements Debug trait

# v0.3.0 (2023-09-14)
- Reduced the size of the TheError type by using NaiveDate and NaiveTime, and integers for location
- Fixed documentation generation errors
- Implemented ``From<SystemErrorCodes>`` for ``TheErrorType``
- Modified the Display format for the ``TheError``
- Added more documentation to functions and traits
- Deleted unused error types that were too specific

# v0.2.6 (2023-09-09)
- Changed required rustc version from 1.72.0 to 1.71.1
- Changed tokio version to 1.29.1
- Changed chrono version to 0.4.26

# v0.2.5 (2023-09-04)
- Added support for std Rust errors
- Changed error type in tokio support 
- Removed conflicting tokio error definition

# v0.2.4 (2023-09-03)
- Added documentation to structs, functions and macros

# v0.2.3 (2023-09-03)
- Added support for tokio crate
- Changed the place where the features are checked for easier readability

# v0.2.2 (2023-09-02)
- Changed the name of the `new_error!` macro into `map_to_new_error!`
- Added support to display errors from the chrono crate
- Updated readme with example

# v0.2.1 (2023-09-02)
- Added support for chrono crate
- Added chrono support feature

# v0.2.0 (2023-09-02)
- Restructured into better suited filenames
- Changed the name of the crate from ´the-error´ to ´error_handler
- Added the compilation features ´mysql_async´ and ´full´ to exclude some errors if needed

# v0.1.0 (2023-09-02)
- Added support for mysql_async errors ´Error´ and its sub-errors