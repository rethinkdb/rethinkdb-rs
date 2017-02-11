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
r.connect([options, ]callback)
r.connect([host, ]callback)
r.connect([options]) &rarr; promise
r.connect([host]) &rarr; promise

{% endapibody %}

# Description #

<img src="/assets/images/docs/api_illustrations/connect_javascript.png" class="api_command_illustration" />

Create a new connection to the database server. Accepts the following
options:

- `host`: the host to connect to (default `localhost`).
- `port`: the port to connect on (default `28015`).
- `db`: the default database (default `test`).
- `user`: the user account to connect as (default `admin`).
- `password`: the password for the user account to connect as (default `''`, empty).
- `timeout`: timeout period in seconds for the connection to be opened (default `20`).
- `ssl`: a hash of options to support SSL connections (default `null`). Currently, there is only one option available, and if the `ssl` option is specified, this key is required:
    - `ca`: a list of [Node.js](http://nodejs.org) `Buffer` objects containing SSL CA certificates.

If the connection cannot be established, a `ReqlDriverError` will be passed to the callback instead of a connection.

<!-- break -->

The returned connection object will have two properties on it containing the connection's port and address:

```js
conn.clientPort;
conn.clientAddress;
```

{% infobox %}
Using SSL with RethinkDB requires proxy software on the server, such as [Nginx][], [HAProxy][] or an SSL tunnel. RethinkDB will encrypt traffic and verify the CA certification to prevent [man-in-the-middle][mitm] attacks. Consult your proxy's documentation for more details.

[Nginx]: http://nginx.org/
[HAProxy]: http://www.haproxy.org/
[mitm]: http://en.wikipedia.org/wiki/Man-in-the-middle_attack

Alternatively, you may use RethinkDB's built-in [TLS support][tls].

[tls]: /docs/security/
{% endinfobox %}

__Example:__ Open a connection using the default host and port, specifying the default database.

```js
r.connect({
    db: 'marvel'
}, function(err, conn) {
    // ...
});
```

If no callback is provided, a promise will be returned.

```js
var promise = r.connect({db: 'marvel'});
```

__Example:__ Open a new connection to the database.

```js
r.connect({
    host: 'localhost',
    port: 28015,
    db: 'marvel'
}, function(err, conn) {
    // ...
});
```

Alternatively, you can use promises.

```js
var p = r.connect({
    host: 'localhost',
    port: 28015,
    db: 'marvel'
});
p.then(function(conn) {
    // ...
}).error(function(error) {
    // ...
});
```

__Example:__ Open a new connection to the database, specifying a user/password combination for authentication.

```js
r.connect({
    host: 'localhost',
    port: 28015,
    db: 'marvel',
    user: 'herofinder',
    password: 'metropolis'
}, function(err, conn) {
    // ...
});
```

__Example:__ Open a new connection to the database using an SSL proxy.

```js
var fs = require('fs');
fs.readFile('/path/to/cert', function (err, caCert) {
    if (!err) {
        r.connect({
            host: 'localhost',
            port: 28015,
            db: 'marvel',
            authKey: 'hunter2',
            ssl: {
                ca: caCert
            }
        }, function(err, conn) {
            // ...
        });
    } else {
        console.log(err);
    }
});
```
