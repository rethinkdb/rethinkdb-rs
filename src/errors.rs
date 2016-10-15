use std::io;
use std::str;
use super::r2d2;
use super::serde_json::error as json;
use protobuf::ProtobufError;

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
        QueryLogic(err: QueryLogicError) { from() }
        ResourceLimit(descr: String)
        User(descr: String)
        Internal(descr: String)
        Timeout(descr: String)
        Availability(err: AvailabilityError) { from() }
        Permissions(descr: String)
    }
}

quick_error! {
    /// The query contains a logical impossibility, such as adding a number to a string.
    #[derive(Debug)]
    pub enum QueryLogicError {
        NonExistence(descr: String)
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
        ParseResponse(err: str::Utf8Error) { from() }
        Json(err: json::Error) { from() }
        Protobuf(err: ProtobufError) { from() }
    }
}

quick_error! {
    /// Connection related errors
    #[derive(Debug)]
    pub enum ConnectionError {
        Initialization(err: r2d2::InitializationError) { from() }
        PoolWrite(descr: String)
        PoolRead(descr: String)
        Timeout(err: r2d2::GetTimeout) { from() }
        Io(err: io::Error) { from() }
        Other(descr: String)
    }
}

/// Converts from r2d2 error
impl From<r2d2::InitializationError> for Error {
    fn from(err: r2d2::InitializationError) -> Error {
        From::from(ConnectionError::Initialization(err))
    }
}

impl From<r2d2::GetTimeout> for Error {
    fn from(err: r2d2::GetTimeout) -> Error {
        From::from(ConnectionError::Timeout(err))
    }
}

impl From<ConnectionError> for Error {
    fn from(err: ConnectionError) -> Error {
        From::from(DriverError::Connection(err))
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
        From::from(DriverError::ParseResponse(err))
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
