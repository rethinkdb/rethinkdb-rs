//! The errors returned by this driver

use std::{io, str};

use super::*;
use serde_json::error as js;

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

impl From<js::Error> for Error {
    fn from(err: js::Error) -> Error {
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
