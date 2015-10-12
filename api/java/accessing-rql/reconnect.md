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
conn.reconnect()
{% endapibody %}

# Description #

Close and reopen a connection.

Closing a connection normally waits until all outstanding requests have finished and then frees any open resources associated with the connection. By passing `false` to the `noreply_wait` [optArg](/api/java/optarg/), the connection will be closed immediately, possibly aborting any outstanding noreply writes.

A noreply query is executed by passing the `noreply` optArg to the [run](/api/java/run/) command, indicating that `run()` should not wait for the query to complete before returning. You may also explicitly wait for a noreply query to complete by using the [noreplyWait](/api/java/noreply_wait) command.

__Example:__ Cancel outstanding requests/queries that are no longer needed.

```java
conn.reconnect().optArg("noreply_wait", false);
```
