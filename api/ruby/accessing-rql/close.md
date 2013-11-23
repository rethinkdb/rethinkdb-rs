---
layout: api-command
language: Ruby
permalink: api/ruby/close/
command: close
related_commands:
    connect: connect/
    use: use/
    repl: repl/
    reconnect: reconnect/
---

# Command syntax #

{% apibody %}
conn.close(opts={})
{% endapibody %}

# Description #

Close an open connection.  Accepts the following options:

- `noreply_wait`: whether to wait for noreply writes to complete
  before closing (default `true`).  If this is set to `false`, some
  outstanding noreply writes may be aborted.

Closing a connection waits until all outstanding requests have
finished and then frees any open resources associated with the
connection.  If `noreply_wait` is set to `false`, all outstanding
requests are canceled immediately.

__Example:__ Close an open connection, waiting for noreply writes to finish.

```rb
conn.close
```

__Example:__ Close an open connection immediately.

```rb
conn.close(:noreply_wait => false)
```
