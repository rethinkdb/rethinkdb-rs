---
layout: api-command
language: JavaScript
permalink: api/javascript/run/
command: run
related_commands:
    connect: connect/
io:
    -   - any
        - null
---

# Command syntax #

{% apibody %}
query.run(conn[, options], callback)
query.run(conn[, options]) &rarr; promise
{% endapibody %}

<img src="/assets/images/docs/api_illustrations/run.png" class="api_command_illustration" />

# Description #

Run a query on a connection. The callback will get either an error, a single JSON
result, or a cursor, depending on the query.

The options can be:

- `readMode`: One of three possible values affecting the consistency guarantee for the query (default: `'single'`).
    - `'single'` (the default) returns values that are in memory (but not necessarily written to disk) on the primary replica.
    - `'majority'` will only return values that are safely committed on disk on a majority of replicas. This requires sending a message to every replica on each read, so it is the slowest but most consistent.
    - `'outdated'` will return values that are in memory on an arbitrarily-selected replica. This is the fastest but least consistent.
- `timeFormat`: what format to return times in (default: `'native'`).
  Set this to `'raw'` if you want times returned as JSON objects for exporting.
- `profile`: whether or not to return a profile of the query's
  execution (default: `false`).
- `durability`: possible values are `'hard'` and `'soft'`. In soft durability mode RethinkDB
will acknowledge the write immediately after receiving it, but before the write has
been committed to disk.
- `groupFormat`: what format to return `grouped_data` and `grouped_streams` in (default: `'native'`).
  Set this to `'raw'` if you want the raw pseudotype.
- `noreply`: set to `true` to not receive the result object or cursor and return immediately.
- `db`: the database to run this query against as a string. The default is the database specified in the `db` parameter to [connect](/api/javascript/connect/) (which defaults to `test`). The database may also be specified with the [db](/api/javascript/db/) command.
- `arrayLimit`: the maximum numbers of array elements that can be returned by a query (default: 100,000). This affects all ReQL commands that return arrays. Note that it has no effect on the size of arrays being _written_ to the database; those always have an upper limit of 100,000 elements.
- `binaryFormat`: what format to return binary data in (default: `'native'`). Set this to `'raw'` if you want the raw pseudotype.
- `minBatchRows`: minimum number of rows to wait for before batching a result set (default: 8). This is an integer.
- `maxBatchRows`: maximum number of rows to wait for before batching a result set (default: unlimited). This is an integer.
- `maxBatchBytes`: maximum number of bytes to wait for before batching a result set (default: 1MB). This is an integer.
- `maxBatchSeconds`: maximum number of seconds to wait before batching a result set (default: 0.5). This is a float (not an integer) and may be specified to the microsecond.
- `firstBatchScaledownFactor`: factor to scale the other parameters down by on the first batch (default: 4). For example, with this set to 8 and `maxBatchRows` set to 80, on the first batch `maxBatchRows` will be adjusted to 10 (80 / 8). This allows the first batch to return faster.

If no callback is provided, a promise will be returned.

__Example:__ Run a query on the connection `conn` and log each row in
the result to the console.

```js
r.table('marvel').run(conn, function(err, cursor) {
    cursor.each(console.log);
})
```

__Example:__ Run a query on the connection `conn` and retrieve all the rows in an
array.

```js
r.table('marvel').run(conn, function(err, cursor) {
    if (err) {
        // process error
    }
    else {
        cursor.toArray(function(err, results) {
            if (err) {
                // process error
            }
            else {
                // process the results
            }
        })
    }
})
```

Alternatively, you can use promises.

```js
r.table('marvel').run(conn).then(function(cursor) {
    return cursor.toArray()
}).then(function(results) {
    // process the results
}).error(function(err) {
    // process error
})
```


__Example:__ If you are OK with potentially out of date data from all
the tables involved in this query and want potentially faster reads,
pass a flag allowing out of date data in an options object. Settings
for individual tables will supercede this global setting for all
tables in the query.

```js
r.table('marvel').run(conn, {readMode: 'outdated'}, function (err, cursor) {
    ...
});
```

__Example:__ If you just want to send a write and forget about it, you
can set `noreply` to true in the options. In this case `run` will
return immediately.

```js
r.table('marvel').run(conn, {noreply: true}, function (err, cursor) {
    ...
});
```

__Example:__ If you want to specify whether to wait for a write to be
written to disk (overriding the table's default settings), you can set
`durability` to `'hard'` or `'soft'` in the options.

```js
r.table('marvel')
    .insert({ superhero: 'Iron Man', superpower: 'Arc Reactor' })
    .run(conn, {noreply: true, durability: 'soft'}, function (err, cursor) {
        ...
    });
```

__Example:__ If you do not want a time object to be converted to a
native date object, you can pass a `time_format` flag to prevent it
(valid flags are "raw" and "native"). This query returns an object
with two fields (`epoch_time` and `$reql_type$`) instead of a native date
object.

```js
r.now().run(conn, {timeFormat: "raw"}, function (err, result) {
    ...
});
```

__Example:__ Specify the database to use for the query.

```js
r.table('marvel').run(conn, {db: 'heroes'}).then(function(cursor) {
    return cursor.toArray()
}).then(function(results) {
    // process the results
}).error(function(err) {
    // process error
})
```

This is equivalent to using the `db` command to specify the database:

```js
r.db('heroes').table('marvel').run(conn) ...
```

__Example:__ Change the batching parameters for this query.

```js
r.table('marvel').run(conn, {
    maxBatchRows: 16,
    maxBatchBytes: 2048
}, callback);
```
