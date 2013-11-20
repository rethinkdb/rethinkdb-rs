---
layout: api-command 
language: JavaScript
permalink: api/javascript/noreply_wait/
command: noreplyWait
related_commands:
    run: run/
    sync: sync/
---

# Command syntax #

{% apibody %}
conn.noreplyWait(callback)
{% endapibody %}

# Description #

Wait for outstanding no-reply requests on the given connection to finish. If you have
previously run queries with the `noreply` flag set, it guarantees that those queries
have been processed by the server.

__Example:__ We have previously run queries with the `noreply` argument set. Now
wait until the server has executed them.

```js
conn.noreplyWait(function(err) { ... })
```

