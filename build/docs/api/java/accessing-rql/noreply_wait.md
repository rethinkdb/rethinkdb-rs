---
layout: api-command 
language: Java
permalink: api/java/noreply_wait/
command: noreplyWait
related_commands:
    run: run/
    sync: sync/
---

# Command syntax #

{% apibody %}
conn.noreplyWait()
{% endapibody %}

# Description #

Ensure that previous queries executed with [runNoReply](/api/java/run_noreply) have been processed by the server. Note that this guarantee only apples to queries run on the same connection.

__Example:__ We have previously executed queries with `runNoReply`. Now wait until the server has processed them.

```java
conn.noreplyWait();
```
