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
conn.close([{noreplyWait: true}, ]callback)
conn.close([{noreplyWait: true}]) &rarr; promise
{% endapibody %}

# Description #

Close an open connection. If no callback is provided, a promise will be returned.

Closing a connection normally waits until all outstanding requests have finished and then frees any open resources associated with the connection. By passing `false` to the `noreply_wait` optional argument, the connection will be closed immediately, possibly aborting any outstanding noreply writes.

A noreply query is executed by passing the `noreply` option to the [run](/api/javascript/run/) command, indicating that `run()` should not wait for the query to complete before returning. You may also explicitly wait for a noreply query to complete by using the [noreplyWait](/api/javascript/noreply_wait) command.

__Example:__ Close an open connection, waiting for noreply writes to finish.

```js
conn.close(function(err) { if (err) throw err; })
```

<!-- stop -->

Alternatively, you can use promises.

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

Alternatively, you can use promises.

```js
conn.close({noreplyWait: false}).then(function() {
    // conn is now closed
}).error(function(err) { 
    // process the error
})
```
