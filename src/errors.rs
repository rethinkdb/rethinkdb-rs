extern crate r2d2;

quick_error! {
    /// The most generic error message in ReQL
    #[derive(Debug)]
    pub enum Error {
        Compile(descr: &'static str) { }
        Runtime(err: RuntimeError) { from() }
        Driver(err: DriverError) { from() }
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
        ResourceLimit(descr: &'static str) {}
        User(descr: &'static str) {}
        Internal(descr: &'static str) {}
        Timeout(descr: &'static str) {}
        Availability(err: AvailabilityError) { from() }
        Permissions(descr: &'static str) {}
    }
}

quick_error! {
    /// The query contains a logical impossibility, such as adding a number to a string.
    #[derive(Debug)]
    pub enum QueryLogicError {
        NonExistence(descr: &'static str) {}
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
        OpFailed(descr: &'static str) {}
        OpIndeterminate(descr: &'static str) {}
    }
}

quick_error! {
    /// An error has occurred within the driver
    ///
    /// This may be a driver bug, or it may be an unfulfillable command, such as an unserializable
    /// query.
    #[derive(Debug)]
    pub enum DriverError {
        Auth(descr: &'static str) {}
        Initialization(err: r2d2::InitializationError) { from() }
        Connection(err: ConnectionError) { from() }
    }
}

quick_error! {
    /// Connection related errors
    #[derive(Debug)]
    pub enum ConnectionError {
        PoolWrite(descr: &'static str) {}
        PoolRead(descr: &'static str) {}
    }
}

/// Converts external error to ours
impl From<r2d2::InitializationError> for Error {
    fn from(err: r2d2::InitializationError) -> Error {
        From::from(DriverError::Initialization(err))
    }
}
