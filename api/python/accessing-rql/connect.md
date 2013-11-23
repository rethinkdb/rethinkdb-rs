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
r.connect(host="localhost", port=28015, db="test", auth_key="", timeout=20)
    &rarr; connection
r.connect(host) &rarr; connection
{% endapibody %}

# Description #

Create a new connection to the database server. The keyword arguments are:

- `host`: host of the RethinkDB instance. The default value is `localhost`.
- `port`: the driver port, by default `28015`.
- `db`: the database used if not explicitly specified in a query, by default `test`.
- `auth_key`: the authentification key, by default the empty string.
- `timeout`: timeout period for the connection to be opened, by default `20` (seconds).


Create a new connection to the database server.

If the connection cannot be established, a `RqlDriverError` exception
will be thrown.

__Example:__ Opens a connection using the default host and port but
specifying the default database.

```py
conn = r.connect(db='marvel')
```
