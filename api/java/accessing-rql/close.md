---
layout: api-command
language: Java
permalink: api/java/close/
command: close
related_commands:
    connect: connect/
    use: use/
---

# Command syntax #

{% apibody %}
conn.close().optarg("noreplyWait", true)
{% endapibody %}

# Description #

Close an open connection.

Closing a connection normally waits until all outstanding requests have finished and then frees any open resources associated with the connection. By passing `false` to the `noreply_wait` optional argument, the connection will be closed immediately, possibly aborting any outstanding noreply writes.

A noreply query is executed by passing the `noreply` option to the [run](/api/java/run/) command, indicating that `run()` should not wait for the query to complete before returning. You may also explicitly wait for a noreply query to complete by using the [noreplyWait](/api/java/noreply_wait) command.

__Example:__ Close an open connection, waiting for noreply writes to finish.

```java
conn.close(function(err) { if (err) throw err; })
```

__Example:__ Close an open connection immediately.

```java
conn.close({noreplyWait: false}, function(err) { if (err) throw err; })
```
