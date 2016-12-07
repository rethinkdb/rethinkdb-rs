//! Error Reference

use std::io;
use std::str;
use r2d2::{InitializationError, GetTimeout};
use serde_json::error as json;
use protobuf::ProtobufError;
use serde_json::Value;
use scram::Error as ScramError;
use futures::sync::mpsc::SendError;

quick_error! {
    /// The most generic error message in ReQL
    #[derive(Debug)]
    pub enum Error {
        Compile(descr: String) {
            display("{}", descr)
        }
        Runtime(err: RuntimeError) {
            from()
            description(err.description())
            cause(err)
            display("{:?}", err)
        }
        Driver(err: DriverError) {
            from()
            description(err.description())
            cause(err)
            display("{:?}", err)
        }
    }
}

quick_error! {
/// The parent class of all runtime errors
///
/// All errors on the server unrelated to compilation. Programs may use this to catch any runtime
/// error, but the server will always return a more specific error class.
    #[derive(Debug)]
    pub enum RuntimeError {
        /// The query contains a logical impossibility, such as adding a number to a string.
        QueryLogic(descr: String)
        NonExistence(descr: String)
        ResourceLimit(descr: String)
        User(descr: String)
        Internal(descr: String)
        Timeout(descr: String)
        Availability(err: AvailabilityError) { from() }
        Permission(descr: String)
    }
}

quick_error! {
    /// A server in the cluster is unavailable
    ///
    /// The parent class of `OpFailedError` and `OpIndeterminateError`. Programs may use this
    /// to catch any availability error, but the server will always return one of this class’s
    /// children.
    #[derive(Debug)]
    pub enum AvailabilityError {
        OpFailed(descr: String)
        OpIndeterminate(descr: String)
    }
}

quick_error! {
    /// An error has occurred within the driver
    ///
    /// This may be a driver bug, or it may be an unfulfillable command, such as an unserializable
    /// query.
    #[derive(Debug)]
    pub enum DriverError {
        Auth(descr: String)
        Connection(err: ConnectionError) { from() }
        Response(err: ResponseError) { from() }
        Lock(err: String)
        Json(err: json::Error) { from() }
        Protobuf(err: ProtobufError) { from() }
        Scram(err: ScramError) { from() }
        Other(descr: String)
    }
}

quick_error! {
    /// Connection related errors
    #[derive(Debug)]
    pub enum ConnectionError {
        Initialization(err: InitializationError) { from() }
        Timeout(err: GetTimeout) { from() }
        Io(err: io::Error) { from() }
        Other(descr: String)
    }
}

quick_error! {
    /// Response related errors
    #[derive(Debug)]
    pub enum ResponseError {
        Parse(err: str::Utf8Error) { from() }
        Db(err: Value) { from() }
    }
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

impl<T> From<SendError<T>> for Error {
    fn from(err: SendError<T>) -> Error {
        let err = format!("{:?}", err);
        From::from(DriverError::Other(err))
    }
}

impl From<AvailabilityError> for Error {
    fn from(err: AvailabilityError) -> Error {
        From::from(RuntimeError::Availability(err))
    }
}

/// Converts from IO error
impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        From::from(ConnectionError::Io(err))
    }
}

/// Converts from Utf8Error error
impl From<str::Utf8Error> for Error {
    fn from(err: str::Utf8Error) -> Error {
        From::from(ResponseError::Parse(err))
    }
}

/// Converts from serde_json error
impl From<json::Error> for Error {
    fn from(err: json::Error) -> Error {
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
