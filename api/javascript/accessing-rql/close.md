---
layout: api-command
language: JavaScript
permalink: api/javascript/close/
command: close
io:
    -   - connection
        - undefined
related_commands:
    connect: connect/
    use: use/
---

# Command syntax #

{% apibody %}
conn.close([opts, ]callback)
{% endapibody %}

# Description #

Close an open connection.  Accepts the following options:

- `noreplyWait`: whether to wait for noreply writes to complete
  before closing (default `true`).  If this is set to `false`, some
  outstanding noreply writes may be aborted.

Closing a connection waits until all outstanding requests have
finished and then frees any open resources associated with the
connection.  If `noreplyWait` is set to `false`, all outstanding
requests are canceled immediately.

__Example:__ Close an open connection, waiting for noreply writes to finish.

```js
conn.close(function(err) { if (err) throw err; })
```

__Example:__ Close an open connection immediately.

```js
conn.close({noreplyWait: false}, function(err) { if (err) throw err; })
```
