---
layout: api-command
language: Python
permalink: api/python/reconnect/
command: reconnect
related_commands:
    connect: connect/
    use: use/
    repl: repl/
---

# Command syntax #

{% apibody %}
conn.reconnect(noreply_wait=True)
{% endapibody %}

# Description #

Close and reopen a connection. Closing a connection waits until all
outstanding requests have finished.  If `noreply_wait` is set to
`false`, all outstanding requests are canceled immediately.

__Example:__ Cancel outstanding requests/queries that are no longer needed.

```py
conn.reconnect(noreply_wait=False)
```
