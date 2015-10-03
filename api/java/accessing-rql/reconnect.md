---
layout: api-command
language: JavaScript
permalink: api/javascript/reconnect/
command: reconnect
related_commands:
    connect: connect/
    use: use/
    close: close/
---

# Command syntax #

{% apibody %}
conn.reconnect([{noreplyWait: true}, ]callback)
conn.reconnect([{noreplyWait: true}]) &rarr; promise
{% endapibody %}

# Description #

Close and reopen a connection.

Closing a connection normally waits until all outstanding requests have finished and then frees any open resources associated with the connection. By passing `false` to the `noreply_wait` optional argument, the connection will be closed immediately, possibly aborting any outstanding noreply writes.

A noreply query is executed by passing the `noreply` option to the [run](/api/javascript/run/) command, indicating that `run()` should not wait for the query to complete before returning. You may also explicitly wait for a noreply query to complete by using the [noreplyWait](/api/javascript/noreply_wait) command.

If no callback is provided, a promise will be returned.

__Example:__ Cancel outstanding requests/queries that are no longer needed.

```js
conn.reconnect({noreplyWait: false}, function(error, connection) { ... })
```

Alternatively, you can use promises.

```js
conn.reconnect({noreplyWait: false}).then(function(conn) {
    // the outstanding queries were canceled and conn is now available again
}).error(function(errror) {
    // process the error
})
```
