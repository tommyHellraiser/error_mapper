use mysql_async::{DriverError, Error, IoError, ServerError, UrlError};
use crate::{SystemErrorCodes, TheErrorType};

#[cfg(feature = "mysql_async")]
impl From<Error> for TheErrorType {
    fn from(value: Error) -> Self {
        let error_type;
        let error_content;
        match value {
            Error::Driver(error) => {
                error_content = error.to_string();
                error_type = SystemErrorCodes::from(error);
            }
            Error::Io(error) => {
                error_content = error.to_string();
                error_type = SystemErrorCodes::from(error);
            }
            Error::Other(error) => {
                error_type = SystemErrorCodes::Other;
                error_content = error.to_string();
            }
            Error::Server(error) => {
                error_content = error.to_string();
                error_type = SystemErrorCodes::from(error);
            }
            Error::Url(error) => {
                error_content = error.to_string();
                error_type = SystemErrorCodes::from(error);
            }
        }
        Self {
            error_type,
            error_content
        }
    }
}

#[cfg(feature = "mysql_async")]
impl From<DriverError> for SystemErrorCodes {
    fn from(value: DriverError) -> Self {
        match value {
            DriverError::CantParseServerVersion { .. } => {SystemErrorCodes::CantParseServerVersion}
            DriverError::ConnectionClosed => {SystemErrorCodes::ConnectionClosed}
            DriverError::FromValue { .. } => {SystemErrorCodes::FromValue}
            DriverError::FromRow { .. } => {SystemErrorCodes::FromRow}
            DriverError::MissingNamedParam { .. } => {SystemErrorCodes::MissingNamedParam}
            DriverError::MixedParams => {SystemErrorCodes::MixedParams}
            DriverError::NamedParamsForPositionalQuery => {SystemErrorCodes::NamedParamsForPositionalQuery}
            DriverError::NestedTransaction => {SystemErrorCodes::NestedTransaction}
            DriverError::PacketOutOfOrder => {SystemErrorCodes::PacketOutOfOrder}
            DriverError::PoolDisconnected => {SystemErrorCodes::PoolDisconnected}
            DriverError::ReadOnlyTransNotSupported => {SystemErrorCodes::ReadOnlyTransNotSupported}
            DriverError::StmtParamsMismatch { .. } => {SystemErrorCodes::StmtParamsMismatch}
            DriverError::UnexpectedPacket { .. } => {SystemErrorCodes::UnexpectedPacket}
            DriverError::UnknownAuthPlugin { .. } => {SystemErrorCodes::UnknownAuthPlugin}
            DriverError::PacketTooLarge => {SystemErrorCodes::PacketTooLarge}
            DriverError::BadCompressedPacketHeader => {SystemErrorCodes::BadCompressedPacketHeader}
            DriverError::NamedPipesDisabled => {SystemErrorCodes::NamedPipesDisabled}
            DriverError::MysqlOldPasswordDisabled => {SystemErrorCodes::MysqlOldPasswordDisabled}
            DriverError::LocalInfile(_) => {SystemErrorCodes::LocalInfile}
            DriverError::NoKeyFound => {SystemErrorCodes::NoKeyFound}
            DriverError::NoClientSslFlagFromServer => {SystemErrorCodes::NoClientSslFlagFromServer}
            DriverError::CleartextPluginDisabled => {SystemErrorCodes::CleartextPluginDisabled}
        }
    }
}

#[cfg(feature = "mysql_async")]
impl From<IoError> for SystemErrorCodes {
    fn from(value: IoError) -> Self {
        match value {
            IoError::Io(_) => {SystemErrorCodes::Io}
            IoError::Tls(_) => {SystemErrorCodes::Tls}
        }
    }
}

#[cfg(feature = "mysql_async")]
impl From<ServerError> for SystemErrorCodes {
    fn from(_: ServerError) -> Self {
        SystemErrorCodes::ServerError
    }
}
#[cfg(feature = "mysql_async")]
impl From<UrlError> for SystemErrorCodes {
    fn from(value: UrlError) -> Self {
        match value {
            UrlError::FeatureRequired { .. } => {SystemErrorCodes::FeatureRequired}
            UrlError::Invalid => {SystemErrorCodes::Invalid}
            UrlError::InvalidParamValue { .. } => {SystemErrorCodes::InvalidParamValue}
            UrlError::InvalidPoolConstraints { .. } => {SystemErrorCodes::InvalidPoolConstraints}
            UrlError::Parse(_) => {SystemErrorCodes::Parse}
            UrlError::UnknownParameter { .. } => {SystemErrorCodes::UnknownParameter}
            UrlError::UnsupportedScheme { .. } => {SystemErrorCodes::Unsupported}
        }
    }
}