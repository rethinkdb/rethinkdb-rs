---
layout: api-command
language: Python
permalink: api/python/index_status/
command: index_status
related_commands:
    index_wait: index_wait/
---

# Command syntax #

{% apibody %}
table.index_status([, index...]) &rarr; array
{% endapibody %}

# Description #

Get the status of the specified indexes on this table, or the status
of all indexes on this table if no indexes are specified.

__Example:__ Get the status of all the indexes on `test`:

```py
r.table('test').index_status().run(conn)
```

__Example:__ Get the status of the `timestamp` index:

```py
r.table('test').index_status('timestamp').run(conn)
```
