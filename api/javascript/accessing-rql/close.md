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
conn.close([opts]) &rarr; promise
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

If no callback is provided, a promise will be returned.

__Example:__ Close an open connection, waiting for noreply writes to finish.

```js
conn.close(function(err) { if (err) throw err; })
```

__Example:__ Close an open connection, waiting for noreply writes to finish, and
using promises

```js
p = conn.close();
p.then(function() {
    // `conn` is now closed
}).error(function(err) {
    // process the error
})
```

__Example:__ Close an open connection immediately.

```js
conn.close({noreplyWait: false}, function(err) { if (err) throw err; })
```

__Example:__ Close an open connection immediately, using promises.

```js
conn.close({noreplyWait: false}).then(function() {
    // conn is now closed
}).error(function(err) { 
    // process the error
})
```
