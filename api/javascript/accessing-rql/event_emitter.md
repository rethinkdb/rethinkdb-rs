---
layout: api-command
language: JavaScript
permalink: api/javascript/event_emitter/
alias:
    - api/javascript/add_listener/
    - api/javascript/on/
    - api/javascript/once/
    - api/javascript/remove_listener/
    - api/javascript/remove_all_listeners/
    - api/javascript/listeners/
    - api/javascript/emit/
command: "EventEmitter's methods"
py: false
rb: false
io:
    -   - connection
        - undefined
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

The connection object supports the event emitter interface so you can listen for
changes in connection state.

__Example:__ Monitor connection state with events 'connect', 'close', and 'error'.


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

__Example:__ As in Node, 'on' is a synonym for 'addListener'.

```js
conn.on('close', function() {
    cleanup();
});
conn.close();
```

