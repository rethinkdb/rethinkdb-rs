---
layout: api-command
language: Java
permalink: api/java/reconnect/
command: reconnect
related_commands:
    connect: connect/
    use: use/
    close: close/
---

# Command syntax #

{% apibody %}
conn.reconnect([boolean, timeout])
{% endapibody %}

# Description #

Close and reopen a connection.

Closing a connection normally waits until all outstanding requests have finished and then frees any open resources associated with the connection. By passing `false` as an optional boolean argument to `reconnect`, the connection will be closed immediately, possibly aborting any outstanding noreply writes. An optional second argument is a (long integer) timeout indicating how long you would like `reconnect` to wait before closing the existing connection.

A noreply query is executed by using the [runNoReply](/api/java/run_noreply/) command, indicating that the command should not wait for the query to complete before returning. You may also explicitly wait for a noreply query to complete by using the [noreplyWait](/api/java/noreply_wait) command.

__Example:__ Cancel outstanding requests/queries that are no longer needed.

```java
conn.reconnect(false);
```

__Example:__ Wait up to 5 seconds for outstanding requests to finish before reconnecting.

```java
conn.reconnect(true, 5);
```
