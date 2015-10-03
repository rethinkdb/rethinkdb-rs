---
layout: api-command
language: Java
permalink: api/javascript/event_emitter/
alias:
    - api/javascript/add_listener/
    - api/javascript/on/
    - api/javascript/once/
    - api/javascript/remove_listener/
    - api/javascript/remove_all_listeners/
    - api/javascript/listeners/
    - api/javascript/emit/
command: "EventEmitter (connection)"
py: false
rb: false
related_commands:
    connect: connect/
    reconnect: reconnect/
    close: close/
---

# Command syntax #

{% apibody %}
connection.addListener(event, listener)
connection.on(event, listener)
connection.once(event, listener)
connection.removeListener(event, listener)
connection.removeAllListeners([event])
connection.setMaxListeners(n)
connection.listeners(event)
connection.emit(event, [arg1], [arg2], [...])
{% endapibody %}

# Description #

Connections implement the same interface as Node's [EventEmitter][ee]. This allows you to listen for changes in connection state.

[ee]: http://nodejs.org/api/events.html#events_class_events_eventemitter

Four events are emitted: `connect`, `close`, `timeout` and `error`.

- `connect`: a successful connection to the server.
- `close`: the connection has been closed, either thorugh an error or by calling `connection.close()`.
- `timeout`: the underlying socket has timed out.
- `error`: a protocol-level error has occurred. (This will *not* be sent on a query error; those are returned to callbacks/promises.)

__Example:__ Monitor the connection state with events.


```js
r.connect({}, function(err, conn) {
    if (err) throw err;

    conn.addListener('error', function(e) {
        processNetworkError(e);
    });

    conn.addListener('close', function() {
        cleanup();
    });

    runQueries(conn);
});
```

__Example:__ As in Node, `on` is a synonym for `addListener`.

```js
conn.on('close', function() {
    cleanup();
});
conn.close();
```

