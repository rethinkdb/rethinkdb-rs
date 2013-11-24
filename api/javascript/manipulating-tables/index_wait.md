---
layout: api-command
language: JavaScript
permalink: api/javascript/index_wait/
command: indexWait
io:
    -   - table
        - array
related_commands:
    indexStatus: index_status/

---

# Command syntax #

{% apibody %}
table.indexWait([, index...]) &rarr; array
{% endapibody %}

# Description #

Wait for the specified indexes on this table to be ready, or for all
indexes on this table to be ready if no indexes are specified.

__Example:__ Wait for all indexes on the table `test` to be ready:

```js
r.table('test').indexWait().run(conn, callback)
```

__Example:__ Wait for the index `timestamp` to be ready:

```js
r.table('test').indexWait('timestamp').run(conn, callback)
```
