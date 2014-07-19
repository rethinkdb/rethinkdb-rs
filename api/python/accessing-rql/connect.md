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

If the connection cannot be established, a `RqlDriverError` exception will be thrown.

__Note:__ Currently, the Python driver is not thread-safe. Each thread or multiprocessing PID should be given its own connection object. (This is likely to change in a future release of RethinkDB; you can track issue [#2427](https://github.com/rethinkdb/rethinkdb/issues/2427) for progress.)

__Example:__ Opens a connection using the default host and port but specifying the default database.

```py
conn = r.connect(db='marvel')
```

__Example:__ Opens a new connection to the database.

```py
conn = r.connect(host = 'localhost',
                 port = 28015,
                 db = 'heroes',
                 auth_key = 'hunter2')
```

