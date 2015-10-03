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
conn.noreplyWait(callback)
conn.noreplyWait() &rarr; promise
{% endapibody %}

# Description #

`noreplyWait` ensures that previous queries with the `noreply` flag have been processed
by the server. Note that this guarantee only applies to queries run on the given connection.

If no callback is provided, a promise will be returned.

__Example:__ We have previously run queries with the `noreply` argument set to `true`. Now
wait until the server has processed them.

```js
conn.noreplyWait(function(err) { ... })
```

Alternatively, you can use promises.

```js
conn.noreplyWait().then(function() {
    // all queries have been processed
}).error(function(err) {
    // process error
})
```

