---
layout: api-command 
language: JavaScript
permalink: api/javascript/connect/
command: connect
io:
    -   - r
        - undefined
related_commands:
    use: use/
    close: close/
---

# Command syntax #

{% apibody %}
r.connect(options, callback)
r.connect(host, callback)
{% endapibody %}

# Description #

Create a new connection to the database server. The available options are:

- `host`: host of the RethinkDB instance. The default value is `localhost`.
- `port`: the driver port, by default `28015`.
- `db`: the database used if not explicitly specified in a query, by default `test`.
- `authKey`: the authentification key, by default the empty string.
- `timeout`: timeout period for the connection to be opened, by default `20` (seconds).


If there is a syntax error, a `RqlDriverError` will be thrown. If the connection cannot
be established, the driver will execute the callback with a `RqlDriverError` exception.


__Example:__ Opens a new connection to the database.

```js
r.connect({
    host: "localhost",
    port: 28015,
    db: "marvel",
    authKey: "hunter2"
}, function(err, conn) { ... })
```

__Example:__ Opens a new connection to the database by just specifying the host.

```js
r.connect("localhost", function(err, conn) { ... })
```
