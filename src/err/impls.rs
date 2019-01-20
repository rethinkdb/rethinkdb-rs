//! The errors returned by this driver

use std::{io, option::NoneError, str};

use super::*;
use futures::channel::mpsc::SendError;
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

impl From<SendError> for Error {
    fn from(_err: SendError) -> Error {
        Driver::Other("message sending failed".to_owned()).into()
    }
}

impl From<NoneError> for Error {
    fn from(_: NoneError) -> Error {
        Driver::Other("expected message, got None".to_owned()).into()
    }
}
