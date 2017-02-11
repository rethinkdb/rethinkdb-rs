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
conn.reconnect([{:noreply_wait => true}])
{% endapibody %}

# Description #

Close and reopen a connection.

Closing a connection normally waits until all outstanding requests have finished and then frees any open resources associated with the connection. By passing `false` to the `noreply_wait` optional argument, the connection will be closed immediately, possibly aborting any outstanding noreply writes.

A noreply query is executed by passing the `noreply` option to the [run](/api/ruby/run/) command, indicating that `run()` should not wait for the query to complete before returning. You may also explicitly wait for a noreply query to complete by using the [noreply_wait](/api/ruby/noreply_wait) command.

__Example:__ Cancel outstanding requests/queries that are no longer needed.

```rb
conn.reconnect(:noreply_wait => false)
```
