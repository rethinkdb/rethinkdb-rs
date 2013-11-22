---
layout: api-command
language: Ruby
permalink: api/ruby/index_wait/
command: index_wait
related_commands:
    index_status: index_status/
---

# Command syntax #

{% apibody %}
table.index_wait([, index...]) &rarr; array
{% endapibody %}

# Description #


Wait for the specified indexes on this table to be ready, or for all
indexes on this table to be ready if no indexes are specified.

__Example:__ Wait for all indexes on the table `test` to be ready:

```js
r.table('test').index_wait.run(conn)
```

__Example:__ Wait for the index `timestamp` to be ready:

```js
r.table('test').index_wait('timestamp').run(conn)
```
