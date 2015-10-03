---
layout: api-command
language: Java
permalink: api/java/index_status/
command: indexStatus
related_commands:
    indexWait: index_wait/

---

# Command syntax #

{% apibody %}
table.indexStatus([, index...]) &rarr; array
{% endapibody %}

# Description #

Get the status of the specified indexes on this table, or the status
of all indexes on this table if no indexes are specified.

The result is an array where for each index, there will be an object like this one:

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

or this one:

```js
{
    index: <indexName>,
    ready: false,
    blocks_processed: <int>,
    blocks_total: <int>,
    function: <binary>,
    multi: <bool>,
    geo: <bool>,
    outdated: <bool>
}
```

The `multi` field will be `true` or `false` depending on whether this index was created as a multi index; the `geo` field will be `true` or `false` depending on whether this index was created as a geospatial index. See [indexCreate](/api/java/index_create/) for details. The `outdated` field will be true if the index is outdated in the current version of RethinkDB and needs to be rebuilt.

The `function` field is a binary object containing an opaque representation of the secondary index (including the `multi` argument if specified). It can be passed as the second argument to [indexCreate](/api/java/index_create/) to create a new index with the same function; see `indexCreate` for more information.

__Example:__ Get the status of all the indexes on `test`:

```js
r.table('test').indexStatus().run(conn)
```

__Example:__ Get the status of the `timestamp` index:

```js
r.table('test').indexStatus('timestamp').run(conn)
```

__Example:__ Save the binary representation of the index:

```js
var func;
r.table('test').indexStatus('timestamp').run(conn, function (err, res) {
    func = res[0].function;
});
```
