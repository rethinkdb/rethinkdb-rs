---
layout: api-command
language: Java
permalink: api/javascript/index_wait/
command: indexWait
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

The result is an array containing one object for each table index:

```js
{
    index: <indexName>,
    ready: true,
    function: <binary>,
    multi: <bool>,
    geo: <bool>,
    outdated: <bool>
}
```

See the [indexStatus](/api/javascript/index_status) documentation for a description of the field values.

__Example:__ Wait for all indexes on the table `test` to be ready:

```js
r.table('test').indexWait().run(conn)
```

__Example:__ Wait for the index `timestamp` to be ready:

```js
r.table('test').indexWait('timestamp').run(conn)
```
