---
layout: api-command
language: Python
permalink: api/python/connect/
command: connect
related_commands:
    use: use/
    repl: repl/
    close: close/
---

# Command syntax #

{% apibody %}
r.connect(host="localhost", port=28015, db="test", auth_key="", timeout=20) &rarr; connection
r.connect(host) &rarr; connection
{% endapibody %}

# Description #

<img src="/assets/images/docs/api_illustrations/connect_python.png" class="api_command_illustration" />

Create a new connection to the database server. The keyword arguments are:

- `host`: host of the RethinkDB instance. The default value is `localhost`.
- `port`: the driver port, by default `28015`.
- `db`: the database used if not explicitly specified in a query, by default `test`.
- `auth_key`: the authentication key, by default the empty string.
- `timeout`: timeout period in seconds for the connection to be opened (default `20`).
- `ssl`: a hash of options to support SSL connections (default `None`). Currently, there is only one option available, and if the `ssl` option is specified, this key is required:
    - `ca_certs`: a path to the SSL CA certificate.

If the connection cannot be established, a `ReqlDriverError` exception will be thrown.

<!-- break -->

{% infobox %}
Using SSL with RethinkDB requires proxy software on the server, such as [Nginx][], [HAProxy][] or an SSL tunnel. RethinkDB will encrypt traffic and verify the CA certification to prevent [man-in-the-middle][mitm] attacks. Consult your proxy's documentation for more details.

[Nginx]: http://nginx.org/
[HAProxy]: http://www.haproxy.org/
[mitm]: http://en.wikipedia.org/wiki/Man-in-the-middle_attack
{% endinfobox %}

The authentication key can be set from the RethinkDB command line tool. Once set, client connections must provide the key as an option to `run` in order to make the connection. For more information, read "Using the RethinkDB authentication system" in the documentation on [securing your cluster](http://rethinkdb.com/docs/security/).

{% infobox alert %}
__Note:__ Currently, the Python driver is not thread-safe. Each thread or multiprocessing PID should be given its own connection object. (This is likely to change in a future release of RethinkDB; you can track issue [#2427](https://github.com/rethinkdb/rethinkdb/issues/2427) for progress.)
{% endinfobox %}

The RethinkDB Python driver includes support for asynchronous connections using Tornado and Twisted. Read the [asynchronous connections][ac] documentation for more information.

[ac]: /docs/async-connections/#python-with-tornado-or-twisted

__Example:__ Open a connection using the default host and port, specifying the default database.

```py
conn = r.connect(db='marvel')
```

__Example:__ Open a new connection to the database.

```py
conn = r.connect(host='localhost',
                 port=28015,
                 db='heroes',
                 auth_key='hunter2')
```

__Example:__ Open a new connection to the database using an SSL proxy.

```py
conn = r.connect(host='localhost',
                 port=28015,
                 auth_key='hunter2',
                 ssl={'ca_certs': '/path/to/ca.crt'})
```

__Example:__ Use a `with` statement to open a connection and pass it to a block. Using this style, the connection will be automatically closed when execution reaches the end of the block.

```py
with r.connect(db='marvel') as conn:
    r.table('superheroes').run(conn)
```
