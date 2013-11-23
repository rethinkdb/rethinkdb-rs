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
{% endapibody %}

# Description #

Close and reopen a connection.  Accepts the following options:

- `noreplyWait`: whether to wait for noreply writes to complete
  before closing (default `true`).  If this is set to `false`, some
  outstanding noreply writes may be aborted.

Closing a connection waits until all outstanding requests have
finished.  If `noreplyWait` is set to `false`, all outstanding
requests are canceled immediately.

__Example:__ Cancel outstanding requests/queries that are no longer needed.

```js
conn.reconnect({noreplyWait: false}, function(errror, connection) { ... })
```
