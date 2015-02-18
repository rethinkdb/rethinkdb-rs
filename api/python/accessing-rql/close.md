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

Close an open connection.

Closing a connection normally waits until all outstanding requests have finished and then frees any open resources associated with the connection. By passing `False` to the `noreply_wait` optional argument, the connection will be closed immediately, possibly aborting any outstanding noreply writes.

A noreply query is executed by passing the `noreply` option to the [run](/api/python/run/) command, indicating that `run()` should not wait for the query to complete before returning. You may also explicitly wait for a noreply query to complete by using the [noreply_wait](/api/python/noreply_wait) command.

__Example:__ Close an open connection, waiting for noreply writes to finish.

```py
conn.close()
```

__Example:__ Close an open connection immediately.

```py
conn.close(noreply_wait=False)
```
