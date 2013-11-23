---
layout: api-command
language: Ruby
permalink: api/ruby/reconnect/
command: reconnect
related_commands:
    connect: connect/
    use: use/
    repl: repl/
    close: close/
---

# Command syntax #

{% apibody %}
conn.reconnect(opts={})
{% endapibody %}

# Description #

Close and reopen a connection.  Accepts the following options:

- `noreply_wait`: whether to wait for noreply writes to complete
  before closing (default `true`).  If this is set to `false`, some
  outstanding noreply writes may be aborted.

Closing a connection waits until all outstanding requests have
finished.  If `noreply_wait` is set to `false`, all outstanding
requests are canceled immediately.

__Example:__ Cancel outstanding requests/queries that are no longer needed.

```rb
conn.reconnect(:noreply_wait => false)
```
