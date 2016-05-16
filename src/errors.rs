use std::error::Error as StdError;
use std::fmt;

/// The most generic error message in ReQL
#[derive(Debug)]
pub enum Error {
    Compile(CompileError),
    Runtime(RuntimeError),
    Driver(DriverError),
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Compile(ref err) => err.description(),
            Error::Runtime(ref err) => err.description(),
            Error::Driver(ref err) => err.description(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Compile(ref err) => err.fmt(f),
            Error::Runtime(ref err) => err.fmt(f),
            Error::Driver(ref err) => err.fmt(f),
        }
    }
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
pub struct CompileError;

impl StdError for CompileError {
    fn description(&self) -> &str {
        "The query cannot be compiled by the server"
    }
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The query cannot be compiled by the server.")
    }
}

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

impl StdError for RuntimeError {
    fn description(&self) -> &str {
        match *self {
            RuntimeError::QueryLogic(ref err) => err.description(),
            RuntimeError::ResourceLimit(ref err) => err.description(),
            RuntimeError::User(ref err) => err.description(),
            RuntimeError::Internal(ref err) => err.description(),
            RuntimeError::Timeout(ref err) => err.description(),
            RuntimeError::Availability(ref err) => err.description(),
            RuntimeError::Permissions(ref err) => err.description(),
        }
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RuntimeError::QueryLogic(ref err) => err.fmt(f),
            RuntimeError::ResourceLimit(ref err) => err.fmt(f),
            RuntimeError::User(ref err) => err.fmt(f),
            RuntimeError::Internal(ref err) => err.fmt(f),
            RuntimeError::Timeout(ref err) => err.fmt(f),
            RuntimeError::Availability(ref err) => err.fmt(f),
            RuntimeError::Permissions(ref err) => err.fmt(f),
        }
    }
}

impl From<QueryLogicError> for RuntimeError {
    fn from(err: QueryLogicError) -> RuntimeError {
        RuntimeError::QueryLogic(err)
    }
}

impl From<ResourceLimitError> for RuntimeError {
    fn from(err: ResourceLimitError) -> RuntimeError {
        RuntimeError::ResourceLimit(err)
    }
}

impl From<UserError> for RuntimeError {
    fn from(err: UserError) -> RuntimeError {
        RuntimeError::User(err)
    }
}

impl From<InternalError> for RuntimeError {
    fn from(err: InternalError) -> RuntimeError {
        RuntimeError::Internal(err)
    }
}

impl From<TimeoutError> for RuntimeError {
    fn from(err: TimeoutError) -> RuntimeError {
        RuntimeError::Timeout(err)
    }
}

impl From<AvailabilityError> for RuntimeError {
    fn from(err: AvailabilityError) -> RuntimeError {
        RuntimeError::Availability(err)
    }
}

impl From<PermissionsError> for RuntimeError {
    fn from(err: PermissionsError) -> RuntimeError {
        RuntimeError::Permissions(err)
    }
}

/// The query contains a logical impossibility, such as adding a number to a string.
#[derive(Debug)]
pub enum QueryLogicError {
    NonExistence(NonExistenceError),
}

impl StdError for QueryLogicError {
    fn description(&self) -> &str {
        match *self {
            QueryLogicError::NonExistence(ref err) => err.description(),
        }
    }
}

impl fmt::Display for QueryLogicError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            QueryLogicError::NonExistence(ref err) => err.fmt(f),
        }
    }
}

impl From<NonExistenceError> for QueryLogicError {
    fn from(err: NonExistenceError) -> QueryLogicError {
        QueryLogicError::NonExistence(err)
    }
}

/// A `QueryLogicError` that results from accessing a non-existent field or something else that
/// can be handled with the default command.
#[derive(Debug)]
pub struct NonExistenceError;

impl StdError for NonExistenceError {
    fn description(&self) -> &str {
        "Non-existence error"
    }
}

impl fmt::Display for NonExistenceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Non-existence error.")
    }
}

/// Query execution caused a resource limit (for example, the array size limit) to be exceeded.
#[derive(Debug)]
pub struct ResourceLimitError;

impl StdError for ResourceLimitError {
    fn description(&self) -> &str {
        "Resource limit error"
    }
}

impl fmt::Display for ResourceLimitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Resource limit error.")
    }
}

/// An error produced by the error command.
#[derive(Debug)]
pub struct UserError;

impl StdError for UserError {
    fn description(&self) -> &str {
        "User error"
    }
}

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "User error.")
    }
}

/// Query execution stopped due to an internal error, i.e., a server bug.
#[derive(Debug)]
pub struct InternalError;

impl StdError for InternalError {
    fn description(&self) -> &str {
        "Server error"
    }
}

impl fmt::Display for InternalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Server error.")
    }
}

/// The query has timed out
///
/// This error happens on the client, not the server. Depending on driver implementation it may
/// derive from a native error class rather than `Error`.
#[derive(Debug)]
pub struct TimeoutError;

impl StdError for TimeoutError {
    fn description(&self) -> &str {
        "Timeout error"
    }
}

impl fmt::Display for TimeoutError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Timeout error.")
    }
}

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

impl StdError for AvailabilityError {
    fn description(&self) -> &str {
        match *self {
            AvailabilityError::OpFailed(ref err) => err.description(),
            AvailabilityError::OpIndeterminate(ref err) => err.description(),
        }
    }
}

impl fmt::Display for AvailabilityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AvailabilityError::OpFailed(ref err) => err.fmt(f),
            AvailabilityError::OpIndeterminate(ref err) => err.fmt(f),
        }
    }
}

impl From<OpFailedError> for AvailabilityError {
    fn from(err: OpFailedError) -> AvailabilityError {
        AvailabilityError::OpFailed(err)
    }
}

impl From<OpIndeterminateError> for AvailabilityError {
    fn from(err: OpIndeterminateError) -> AvailabilityError {
        AvailabilityError::OpIndeterminate(err)
    }
}

/// The operation has failed due to cluster state, configuration or table availability.
#[derive(Debug)]
pub struct OpFailedError;

impl StdError for OpFailedError {
    fn description(&self) -> &str {
        "Operation failed error"
    }
}

impl fmt::Display for OpFailedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Operation failed error.")
    }
}

/// The status of the operation cannot be verified due to cluster state, configuration or table
/// availability.
#[derive(Debug)]
pub struct OpIndeterminateError;

impl StdError for OpIndeterminateError {
    fn description(&self) -> &str {
        "Operation cannot be verified"
    }
}

impl fmt::Display for OpIndeterminateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Operation cannot be verified.")
    }
}

/// The user account does not have the permissions necessary to execute the query.
#[derive(Debug)]
pub struct PermissionsError;

impl StdError for PermissionsError {
    fn description(&self) -> &str {
        "Permission error"
    }
}

impl fmt::Display for PermissionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Permission error.")
    }
}

/// An error has occurred within the driver
///
/// This may be a driver bug, or it may be an unfulfillable command, such as an unserializable
/// query.
#[derive(Debug)]
pub enum DriverError {
    Auth(AuthError),
}

impl StdError for DriverError {
    fn description(&self) -> &str {
        match *self {
            DriverError::Auth(ref err) => err.description(),
        }
    }
}

impl fmt::Display for DriverError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DriverError::Auth(ref err) => err.fmt(f),
        }
    }
}

impl From<AuthError> for DriverError {
    fn from(err: AuthError) -> DriverError {
        DriverError::Auth(err)
    }
}

/// The client failed authentication with the server.
#[derive(Debug)]
pub struct AuthError;

impl StdError for AuthError {
    fn description(&self) -> &str {
        "The client failed authentication with the server"
    }
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The client failed authentication with the server.")
    }
}

