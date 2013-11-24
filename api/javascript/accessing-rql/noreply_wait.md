---
layout: api-command 
language: JavaScript
permalink: api/javascript/noreply_wait/
command: noreplyWait
io:
  - - connection
    - null
related_commands:
    run: run/
    sync: sync/
---

# Command syntax #

{% apibody %}
conn.noreplyWait(callback)
{% endapibody %}

# Description #

`noreplyWait` ensures that previous queries with the `noreply` flag have been processed
by the server. Note that this guarantee only applies to queries run on the given connection.

__Example:__ We have previously run queries with the `noreply` argument set to `true`. Now
wait until the server has processed them.

```js
conn.noreplyWait(function(err) { ... })
```

