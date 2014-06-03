---
layout: api-command
language: JavaScript
permalink: api/javascript/reconnect/
command: reconnect
io:
    -   - connection
        - undefined
related_commands:
    connect: connect/
    use: use/
    close: close/
---

# Command syntax #

{% apibody %}
conn.reconnect([opts, ]callback)
conn.reconnect([opts, ]) &rarr; promise
{% endapibody %}

# Description #

Close and reopen a connection.  Accepts the following options:

- `noreplyWait`: whether to wait for noreply writes to complete
  before closing (default `true`).  If this is set to `false`, some
  outstanding noreply writes may be aborted.

Closing a connection waits until all outstanding requests have
finished.  If `noreplyWait` is set to `false`, all outstanding
requests are canceled immediately.

If no callback is provided, a promise will be returned.

__Example:__ Cancel outstanding requests/queries that are no longer needed.

```js
conn.reconnect({noreplyWait: false}, function(errror, connection) { ... })
```

__Example:__ Alternatively, you can use promises.

```js
conn.reconnect({noreplyWait: false}).then(function(conn) {
    // the outstanding queries were canceled and conn is now available again
}).error(function(errror) {
    // process the error
})
```
