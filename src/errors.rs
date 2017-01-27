//! Error Reference

use std::io::Error as IoError;
use std::str::Utf8Error;
use std::sync::mpsc::SendError;
use r2d2::{InitializationError, GetTimeout};
use serde_json::error::Error as JsonError;
use protobuf::ProtobufError;
use serde_json::Value;
use scram::Error as ScramError;

/// The most generic error message in ReQL
#[derive(Debug, Error)]
pub enum Error {
    #[error(msg_embedded, non_std, no_from)]
    Compile(String),
    Runtime(RuntimeError),
    Driver(DriverError),
}

/// The parent class of all runtime errors
///
/// All errors on the server unrelated to compilation. Programs may use this to catch any runtime
/// error, but the server will always return a more specific error class.
#[derive(Debug, Error)]
pub enum RuntimeError {
    /// The query contains a logical impossibility, such as adding a number to a string.
    #[error(msg_embedded, non_std, no_from)]
    QueryLogic(String),
    #[error(msg_embedded, non_std, no_from)]
    NonExistence(String),
    #[error(msg_embedded, non_std, no_from)]
    ResourceLimit(String),
    #[error(msg_embedded, non_std, no_from)]
    User(String),
    #[error(msg_embedded, non_std, no_from)]
    Internal(String),
    #[error(msg_embedded, non_std, no_from)]
    Timeout(String),
    Availability(AvailabilityError),
    #[error(msg_embedded, non_std, no_from)]
    Permission(String),
}

/// A server in the cluster is unavailable
///
/// The parent class of `OpFailedError` and `OpIndeterminateError`. Programs may use this
/// to catch any availability error, but the server will always return one of this class’s
/// children.
#[derive(Debug, Error)]
pub enum AvailabilityError {
    #[error(msg_embedded, non_std, no_from)]
    OpFailed(String),
    #[error(msg_embedded, non_std, no_from)]
    OpIndeterminate(String),
}

/// An error has occurred within the driver
///
/// This may be a driver bug, or it may be an unfulfillable command, such as an unserializable
/// query.
#[derive(Debug, Error)]
pub enum DriverError {
    #[error(msg_embedded, non_std, no_from)]
    Auth(String),
    Connection(ConnectionError),
    Response(ResponseError),
    Json(JsonError),
    Protobuf(ProtobufError),
    Scram(ScramError),
    #[error(msg_embedded, non_std, no_from)]
    Other(String),
}

/// Connection related errors
#[derive(Debug, Error)]
pub enum ConnectionError {
    Initialization(InitializationError),
    Timeout(GetTimeout),
    Io(IoError),
    #[error(msg_embedded, non_std, no_from)]
    Other(String),
}

/// Response related errors
#[derive(Debug, Error)]
pub enum ResponseError {
    Parse(Utf8Error),
    #[error(non_std, no_from)]
    Db(Value),
}

/// Converts from r2d2 error
impl From<InitializationError> for Error {
    fn from(err: InitializationError) -> Error {
        From::from(ConnectionError::Initialization(err))
    }
}

impl From<GetTimeout> for Error {
    fn from(err: GetTimeout) -> Error {
        From::from(ConnectionError::Timeout(err))
    }
}

impl From<ConnectionError> for Error {
    fn from(err: ConnectionError) -> Error {
        From::from(DriverError::Connection(err))
    }
}

impl From<ResponseError> for Error {
    fn from(err: ResponseError) -> Error {
        From::from(DriverError::Response(err))
    }
}

impl From<AvailabilityError> for Error {
    fn from(err: AvailabilityError) -> Error {
        From::from(RuntimeError::Availability(err))
    }
}

/// Converts from IO error
impl From<IoError> for Error {
    fn from(err: IoError) -> Error {
        From::from(ConnectionError::Io(err))
    }
}

/// Converts from Utf8Error error
impl From<Utf8Error> for Error {
    fn from(err: Utf8Error) -> Error {
        From::from(ResponseError::Parse(err))
    }
}

/// Converts from serde_json error
impl From<JsonError> for Error {
    fn from(err: JsonError) -> Error {
        From::from(DriverError::Json(err))
    }
}

impl From<ProtobufError> for Error {
    fn from(err: ProtobufError) -> Error {
        From::from(DriverError::Protobuf(err))
    }
}

impl From<ScramError> for Error {
    fn from(err: ScramError) -> Error {
        From::from(DriverError::Scram(err))
    }
}

impl<T> From<SendError<T>> for Error {
    fn from(err: SendError<T>) -> Error {
        let msg = format!("{:?}", err);
        From::from(DriverError::Other(msg))
    }
}
