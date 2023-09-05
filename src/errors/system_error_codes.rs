use std::fmt::{Display, Formatter};

#[derive(Debug)]
#[allow(dead_code)]
pub enum SystemErrorCodes {
    AddrInUse,
    AddrNotAvailable,
    AlreadyExists,
    ArgumentListTooLong,
    BadDateFormat,
    BadCompressedPacketHeader,
    BorrowError,
    BrokenPipe,
    CantParseServerVersion,
    ChannelError,
    CleartextPluginDisabled,
    ConnectionAborted,
    ConnectionClosed,
    ConnectionRefused,
    ConnectionReset,
    ConversionError,
    CrossesDevices,
    DbConnectionError,
    DbConnectionTimedOut,
    Deadlock,
    DecodeError,
    DirectoryNotEmpty,
    Empty,
    EnvironmentError,
    ExecutableFileBusy,
    FeatureRequired,
    FileError,
    FileTooLarge,
    FilesystemLoop,
    FilesystemQuotaExceeded,
    FormatError,
    FromRow,
    FromValue,
    GenericError,
    HandleError,
    HostUnreachable,
    InitializeError,
    Interrupted,
    Invalid,
    InvalidDateValue,
    InvalidData,
    InvalidFilename,
    InvalidFilepath,
    InvalidInput,
    InvalidParamValue,
    InvalidPoolConstraints,
    Io,
    IsADirectory,
    JoinError,
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
    NetworkDown,
    NetworkUnreachable,
    NoClientSslFlagFromServer,
    NoKeyFound,
    NotADirectory,
    NotConnected,
    NotFound,
    NotSeekable,
    Other,
    OutOfMemory,
    OutOfRangeDateValue,
    Overflow,
    PacketOutOfOrder,
    PacketTooLarge,
    Parse,
    PermissionDenied,
    PoolDisconnected,
    ReadOnlyFilesystem,
    ReadOnlyTransNotSupported,
    ReadWriteError,
    ResourceBusy,
    SemaphoreError,
    ServerError,
    SocketError,
    SyncError,
    SlowConnection,
    StaleNetworkFileHandle,
    StmtParamsMismatch,
    StorageFull,
    SystemTimeError,
    TaskError,
    ThreadError,
    TimedOut,
    TimerError,
    Tls,
    TooManyLinks,
    Uncategorized,
    UnexpectedEof,
    UnexpectedPacket,
    UnknownAuthPlugin,
    UnknownParameter,
    UnstableErrorType,
    Unsupported,
    UnsupportedScheme,
    WouldBlock,
    WriteZero,
    Zero
}

impl Display for SystemErrorCodes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
