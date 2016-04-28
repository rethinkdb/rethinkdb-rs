//! ReQL Error Messages
//!
//! These are the error messages returned by the driver. See their [documentation] for details.
//!
//! [documentation]: https://www.rethinkdb.com/docs/error-types/

/// The most generic error message in ReQL
#[derive(Debug)]
pub enum ReqlError {
    Compile(ReqlCompileError),
    Runtime(ReqlRuntimeError),
    Driver(ReqlDriverError),
}

/// The query cannot be compiled by the server
///
/// This may be due to a syntax error, such as an unrecognized optional argument, or specifying the
/// wrong number of arguments to a command.
#[derive(Debug)]
pub enum ReqlCompileError {}

/// The parent class of all runtime errors
///
/// All errors on the server unrelated to compilation. Programs may use this to catch any runtime
/// error, but the server will always return a more specific error class.
#[derive(Debug)]
pub enum ReqlRuntimeError {
    QueryLogic(ReqlQueryLogicError),
    ResourceLimit(ReqlResourceLimitError),
    User(ReqlUserError),
    Internal(ReqlInternalError),
    Timeout(ReqlTimeoutError),
    Availability(ReqlAvailabilityError),
    Permissions(ReqlPermissionsError),
}

/// The query contains a logical impossibility, such as adding a number to a string.
#[derive(Debug)]
pub enum ReqlQueryLogicError {
    NonExistence(ReqlNonExistenceError),
}

/// A `ReqlQueryLogicError` that results from accessing a non-existent field or something else that
/// can be handled with the default command.
#[derive(Debug)]
pub enum ReqlNonExistenceError {}

/// Query execution caused a resource limit (for example, the array size limit) to be exceeded.
#[derive(Debug)]
pub enum ReqlResourceLimitError {}

/// An error produced by the error command.
#[derive(Debug)]
pub enum ReqlUserError {}

/// Query execution stopped due to an internal error, i.e., a server bug.
#[derive(Debug)]
pub enum ReqlInternalError {}

/// The query has timed out
///
/// This error happens on the client, not the server. Depending on driver implementation it may
/// derive from a native error class rather than `ReqlError`.
#[derive(Debug)]
pub enum ReqlTimeoutError {}

/// A server in the cluster is unavailable
///
/// The parent class of `ReqlOpFailedError` and `ReqlOpIndeterminateError`. Programs may use this
/// to catch any availability error, but the server will always return one of this class’s
/// children.
#[derive(Debug)]
pub enum ReqlAvailabilityError {
    OpFailed(ReqlOpFailedError),
    OpIndeterminate(ReqlOpIndeterminateError),
}

/// The operation has failed due to cluster state, configuration or table availability.
#[derive(Debug)]
pub enum ReqlOpFailedError {}

/// The status of the operation cannot be verified due to cluster state, configuration or table
/// availability.
#[derive(Debug)]
pub enum ReqlOpIndeterminateError {}

/// The user account does not have the permissions necessary to execute the query.
#[derive(Debug)]
pub enum ReqlPermissionsError {}

/// An error has occurred within the driver
///
/// This may be a driver bug, or it may be an unfulfillable command, such as an unserializable
/// query.
#[derive(Debug)]
pub enum ReqlDriverError {
    Auth(ReqlAuthError),
}

/// The client failed authentication with the server.
#[derive(Debug)]
pub enum ReqlAuthError {}
