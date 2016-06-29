---
layout: documentation
title: ReQL error types
active: docs
docs_active: error-types
permalink: docs/error-types/
---

RethinkDB has three classes of errors: driver (`ReqlDriverError`), query compilation (`ReqlCompileError`) and runtime (errors subclassed from `ReqlRuntimeError`).

{% toctag %}

# Error types

`ReqlCompileError`: the query cannot be compiled by the server. This may be due to a syntax error, such as an unrecognized optional argument, or specifying the wrong number of arguments to a command. __Note:__ Some drivers may catch certain syntax errors and return a `ReqlDriverError` before sending the query to the server.

`ReqlRuntimeError`: the parent class of all runtime errors (all errors on the server unrelated to compilation). Programs may use this to catch any runtime error, but the server will always return a more specific error class.

`ReqlQueryLogicError`: the query contains a logical impossibility, such as adding a number to a string.

`ReqlNonExistenceError`: a `ReqlQueryLogicError` that results from accessing a non-existent field or something else that can be handled with the [default][] command.

`ReqlResourceLimitError`: query execution caused a resource limit (for example, the array size limit) to be exceeded.

`ReqlTimeoutError`: the query has timed out. (This error happens on the client, not the server. Depending on driver implementation it may derive from a native error class rather than `ReqlError`.)

`ReqlAvailabilityError`: the parent class of `ReqlOpFailedError` and `ReqlOpIndeterminateError`, indicating that a server in the cluster is unavailable. Programs may use this to catch any availability error, but the server will always return one of this class's children.

`ReqlOpFailedError`: the operation has failed due to cluster state, configuration or table availability.

`ReqlOpIndeterminateError`: the status of the operation cannot be verified due to cluster state, configuration or table availability.

`ReqlUserError`: an error produced by the [error][] command.

`ReqlInternalError`: query execution stopped due to an internal error, i.e., a server bug.

`ReqlDriverError`: an error has occurred within the driver. This may be a driver bug, or it may be an unfulfillable command, such as an unserializable query.

`ReqlPermissionsError`: the user account does not have the permissions necessary to execute the query. See [Permissions and user accounts][pua] for more information.

`ReqlAuthError`: the client failed authentication with the server. This is a subclass of `ReqlDriverError`.

[default]: /api/javascript/default/
[error]: /api/javascript/error/
[pua]: /docs/permissions-and-accounts/

# Hierarchy

All errors are subclassed from the `ReqlError` class.

* `ReqlError`
    * `ReqlCompileError`
    * `ReqlRuntimeError`
        * `ReqlQueryLogicError`
            * `ReqlNonExistenceError`
        * `ReqlResourceLimitError`
        * `ReqlUserError`
        * `ReqlInternalError`
        * `ReqlTimeoutError`
        * `ReqlAvailabilityError`
            * `ReqlOpFailedError`
            * `ReqlOpIndeterminateError`
        * `ReqlPermissionsError`
    * `ReqlDriverError`
        * `ReqlAuthError`
