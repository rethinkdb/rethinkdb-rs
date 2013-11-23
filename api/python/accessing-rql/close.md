---
layout: api-command
language: Python
permalink: api/python/close/
command: close
related_commands:
    connect: connect/
    use: use/
    repl: repl/
    reconnect: reconnect/
---

# Command syntax #


{% apibody %}
conn.close(noreply_wait=True)
{% endapibody %}

# Description #

Close an open connection. Closing a connection waits until all
outstanding requests have finished and then frees any open resources
associated with the connection.  If `noreply_wait` is set to `false`,
all outstanding requests are canceled immediately.

Closing a connection cancels all outstanding requests and frees the
memory associated with any open cursors.

__Example:__ Close an open connection, waiting for noreply writes to finish.

```py
conn.close()
```

__Example:__ Close an open connection immediately.

```py
conn.close(noreply_wait=False)
```
