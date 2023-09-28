use std::fmt::{Display, Formatter};
use crate::TheErrorType;

/// Main enum that contains all the possible **error types** that can be mapped from the
/// supported crate's **origin errors**
#[derive(Debug)]
#[allow(dead_code)]
pub enum SystemErrorCodes {
    BadDateFormat,
    BadCompressedPacketHeader,
    BorrowError,
    CantParseServerVersion,
    ChannelError,
    CleartextPluginDisabled,
    ConnectionClosed,
    ConnectionError,
    ConnectionRefused,
    ConversionError,
    DbConnectionError,
    DbConnectionTimedOut,
    Deadlock,
    DecodeError,
    Empty,
    EnvironmentError,
    FeatureRequired,
    FormatError,
    FromRow,
    FromValue,
    GenericError,
    HandleError,
    HostUnreachable,
    HttpRequestError,
    HttpResponseError,
    InitializeError,
    Invalid,
    InvalidDateValue,
    InvalidData,
    InvalidFilename,
    InvalidFilepath,
    InvalidInput,
    InvalidParamValue,
    InvalidPoolConstraints,
    Io,
    JoinError,
    JsonError,
    LayoutError,
    LocalInfile,
    LockError,
    MemoryError,
    MissingNamedParam,
    MixedParams,
    MutexError,
    MysqlOldPasswordDisabled,
    NamedParamsForPositionalQuery,
    NamedPipesDisabled,
    NestedTransaction,
    NoClientSslFlagFromServer,
    NoKeyFound,
    NotConnected,
    NotFound,
    Other,
    OutOfRange,
    OutOfRangeDateValue,
    Overflow,
    PacketOutOfOrder,
    PacketTooLarge,
    ParseError,
    PermissionDenied,
    PoolDisconnected,
    ReadOnlyTransNotSupported,
    ReadWriteError,
    ResourceBusy,
    SemaphoreError,
    ServerError,
    SerializationError,
    SocketError,
    SyncError,
    SlowConnection,
    StmtParamsMismatch,
    StorageFull,
    SystemTimeError,
    TaskError,
    ThreadError,
    TimedOut,
    TimerError,
    Tls,
    UnexpectedPacket,
    UnknownAuthPlugin,
    UnknownParameter,
    Unsupported,
    UrlError,
    Zero
}

/// Display implementation for the **SystemErrorCodes** enum
///
/// Can't display a specific error message because some of the errors contained in this enum
/// are derived from **multiple different error origins**.
///
/// Displaying a SystemErrorCodes enum will display the **error type** as a String instead
impl Display for SystemErrorCodes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Conversion from **SystemErrorCodes** to **TheErrorType** to generate a **TheError** struct
/// using the **map_to_new_error!** macro
impl From<SystemErrorCodes> for TheErrorType {
    fn from(error: SystemErrorCodes) -> Self {
        TheErrorType {
            error_content: error.to_string(),
            error_type: error,
        }
    }
}