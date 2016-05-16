/// The most generic error message in ReQL
#[derive(Debug)]
pub enum Error {
    Compile(CompileError),
    Runtime(RuntimeError),
    Driver(DriverError),
}

impl From<CompileError> for Error {
    fn from(err: CompileError) -> Error {
        Error::Compile(err)
    }
}

impl From<RuntimeError> for Error {
    fn from(err: RuntimeError) -> Error {
        Error::Runtime(err)
    }
}

impl From<DriverError> for Error {
    fn from(err: DriverError) -> Error {
        Error::Driver(err)
    }
}

/// The query cannot be compiled by the server
///
/// This may be due to a syntax error, such as an unrecognized optional argument, or specifying the
/// wrong number of arguments to a command.
#[derive(Debug)]
pub enum CompileError {}

/// The parent class of all runtime errors
///
/// All errors on the server unrelated to compilation. Programs may use this to catch any runtime
/// error, but the server will always return a more specific error class.
#[derive(Debug)]
pub enum RuntimeError {
    QueryLogic(QueryLogicError),
    ResourceLimit(ResourceLimitError),
    User(UserError),
    Internal(InternalError),
    Timeout(TimeoutError),
    Availability(AvailabilityError),
    Permissions(PermissionsError),
}

/// The query contains a logical impossibility, such as adding a number to a string.
#[derive(Debug)]
pub enum QueryLogicError {
    NonExistence(NonExistenceError),
}

/// A `QueryLogicError` that results from accessing a non-existent field or something else that
/// can be handled with the default command.
#[derive(Debug)]
pub enum NonExistenceError {}

/// Query execution caused a resource limit (for example, the array size limit) to be exceeded.
#[derive(Debug)]
pub enum ResourceLimitError {}

/// An error produced by the error command.
#[derive(Debug)]
pub enum UserError {}

/// Query execution stopped due to an internal error, i.e., a server bug.
#[derive(Debug)]
pub enum InternalError {}

/// The query has timed out
///
/// This error happens on the client, not the server. Depending on driver implementation it may
/// derive from a native error class rather than `Error`.
#[derive(Debug)]
pub enum TimeoutError {}

/// A server in the cluster is unavailable
///
/// The parent class of `OpFailedError` and `OpIndeterminateError`. Programs may use this
/// to catch any availability error, but the server will always return one of this class’s
/// children.
#[derive(Debug)]
pub enum AvailabilityError {
    OpFailed(OpFailedError),
    OpIndeterminate(OpIndeterminateError),
}

/// The operation has failed due to cluster state, configuration or table availability.
#[derive(Debug)]
pub enum OpFailedError {}

/// The status of the operation cannot be verified due to cluster state, configuration or table
/// availability.
#[derive(Debug)]
pub enum OpIndeterminateError {}

/// The user account does not have the permissions necessary to execute the query.
#[derive(Debug)]
pub enum PermissionsError {}

/// An error has occurred within the driver
///
/// This may be a driver bug, or it may be an unfulfillable command, such as an unserializable
/// query.
#[derive(Debug)]
pub enum DriverError {
    Auth(AuthError),
}

/// The client failed authentication with the server.
#[derive(Debug)]
pub enum AuthError {}
