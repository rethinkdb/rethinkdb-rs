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
r.connect(options) &rarr; promise
r.connect(host) &rarr; promise

{% endapibody %}

# Description #

<img src="/assets/images/docs/api_illustrations/connect_javascript.png" class="api_command_illustration" />

Create a new connection to the database server.  Accepts the following
options:

- `host`: the host to connect to (default `localhost`).
- `port`: the port to connect on (default `28015`).
- `db`: the default database (default `test`).
- `authKey`: the authentication key (default none).

If the connection cannot be established, a `RqlDriverError` will be passed to the callback instead of a connection.

__Example:__ Opens a connection using the default host and port but specifying the default database.

```js
conn = r.connect({ db: 'marvel' },
                 function(err, conn) { ... })
```

If no callback is provided, a promise will be returned.

__Example:__ Opens a new connection to the database.

```js
r.connect({ host: 'localhost',
            port: 28015,
            db: 'marvel',
            authKey: 'hunter2' },
          function(err, conn) { ... })
```

__Example:__ Opens a new connection to the database and using promises.

```js
var p = r.connect({host:'localhost', port:28015, db:'marvel', authKey:'hunter2'});
p.then(function(conn) {
    // ...
}).error(function(error) {
    // ...
})
```
