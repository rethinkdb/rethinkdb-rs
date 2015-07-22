---
layout: documentation
title: ReQL error types
active: docs
docs_active: error-types
permalink: docs/error-types/
---

RethinkDB's error hierarchy is subclassed from a single error type, `ReqlError`.

* `ReqlCompileError`
* `ReqlRuntimeError`
    * `ReqlQueryLogicError`
        * `ReqlNonExistenceError`
    * `ReqlResourceLimitError`
    * `ReqlUserError`
    * `ReqlInternalError`
    * `ReqlTimeoutError` (client side)
    * `ReqlAvailabilityError`
        * `ReqlOpFailedError`
        * `ReqlOpIndeterminateError`
* `ReqlDriverError`
    * `ReqlAuthError`

- ReqlCursorEmpty (client side, Python)
