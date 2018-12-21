//! The errors returned by this driver

use std::io;

use serde_json::error as js;

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

    Scram(scram::Error),
    Io(io::Error),

    Json(js::Error),
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
        From::from(Runtime::Availability(err))
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        From::from(Driver::Io(err))
    }
}

impl From<js::Error> for Error {
    fn from(err: js::Error) -> Error {
        From::from(Driver::Json(err))
    }
}

impl From<scram::Error> for Error {
    fn from(err: scram::Error) -> Error {
        From::from(Driver::Scram(err))
    }
}
