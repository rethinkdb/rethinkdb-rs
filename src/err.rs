use std::{io, str};

/// The most generic error message in ReQL
#[derive(Debug)]
pub enum Error {
    Compile(String),
    Runtime(Runtime),
    Driver(Driver),
}

/// The parent class of all runtime errors
///
/// All errors on the server unrelated to compilation. Programs may use this to catch any runtime
/// error, but the server will always return a more specific error class.
#[derive(Debug)]
pub enum Runtime {
    /// The query contains a logical impossibility, such as adding a number to a string.
    QueryLogic(String),
    NonExistence(String),
    ResourceLimit(String),
    User(String),
    Internal(String),
    Timeout(String),
    Availability(Availability),
    Permission(String),
}

/// A server in the cluster is unavailable
///
/// The parent class of `OpFailedError` and `OpIndeterminateError`. Programs may use this
/// to catch any availability error, but the server will always return one of this class’s
/// children.
#[derive(Debug)]
pub enum Availability {
    OpFailed(String),
    OpIndeterminate(String),
}

/// An error has occurred within the driver
///
/// This may be a driver bug, or it may be an unfulfillable command, such as an unserializable
/// query.
#[derive(Debug)]
pub enum Driver {
    Auth(String),
    Utf8(str::Utf8Error),
    Scram(scram::Error),
    Io(io::Error),
    Json(serde_json::Error),
    ConnectionBroken,
    Other(String),
}

impl From<Driver> for Error {
    fn from(err: Driver) -> Error {
        Error::Driver(err)
    }
}

impl From<Runtime> for Error {
    fn from(err: Runtime) -> Error {
        Error::Runtime(err)
    }
}

impl From<Availability> for Error {
    fn from(err: Availability) -> Error {
        Runtime::Availability(err).into()
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Driver::Io(err).into()
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Driver::Json(err).into()
    }
}

impl From<scram::Error> for Error {
    fn from(err: scram::Error) -> Error {
        Driver::Scram(err).into()
    }
}

impl From<str::Utf8Error> for Error {
    fn from(err: str::Utf8Error) -> Error {
        Driver::Utf8(err).into()
    }
}
