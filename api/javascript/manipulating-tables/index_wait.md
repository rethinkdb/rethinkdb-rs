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

The result is an array where for each index, there will be an object like:

```js
{
    index: <indexName>,
    ready: true,
    multi: <bool>,
    outdated: <bool>
}
```

The `multi` field will be `true` or `false` depending on whether this index was created as a multi index (see [indexCreate](/api/javascript/index_create/) for details). The `outdated` field will be true if the index is outdated in the current version of RethinkDB and needs to be rebuilt.

__Example:__ Wait for all indexes on the table `test` to be ready:

```js
r.table('test').indexWait().run(conn, callback)
```

__Example:__ Wait for the index `timestamp` to be ready:

```js
r.table('test').indexWait('timestamp').run(conn, callback)
```
