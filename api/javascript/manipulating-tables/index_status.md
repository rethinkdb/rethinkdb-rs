---
layout: api-command
language: JavaScript
permalink: api/javascript/index_status/
command: indexStatus
io:
    -   - table
        - array
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
    progress: <float>,
    function: <binary>,
    multi: <bool>,
    geo: <bool>,
    outdated: <bool>
}
```

The `multi` field will be `true` or `false` depending on whether this index was created as a multi index; the `geo` field will be `true` or `false` depending on whether this index was created as a geospatial index. See [indexCreate](/api/javascript/index_create/) for details. The `outdated` field will be true if the index is outdated in the current version of RethinkDB and needs to be rebuilt. The `progress` field is a float between `0` and `1`, indicating how far along the server is in constructing indexes after the most recent change to the table that would affect them. (`0` indicates no such indexes have been constructed; `1` indicates all of them have.)

The `function` field is a binary object containing an opaque representation of the secondary index (including the `multi` argument if specified). It can be passed as the second argument to [indexCreate](/api/javascript/index_create/) to create a new index with the same function; see `indexCreate` for more information.

__Example:__ Get the status of all the indexes on `test`:

```js
r.table('test').indexStatus().run(conn, callback)
```

__Example:__ Get the status of the `timestamp` index:

```js
r.table('test').indexStatus('timestamp').run(conn, callback)
```

__Example:__ Save the binary representation of the index:

```js
var func;
r.table('test').indexStatus('timestamp').run(conn, function (err, res) {
    func = res[0].function;
});
```
