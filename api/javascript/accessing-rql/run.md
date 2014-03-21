---
layout: api-command
language: JavaScript
permalink: api/javascript/run/
command: run
io:
    -   - any
        - null
---

# Command syntax #

{% apibody %}
query.run(conn[, options], callback)
{% endapibody %}

# Description #

Run a query on a connection. The callback will get either an error, a single JSON
result, or a cursor, depending on the query.

The options can be:

- `useOutdated`: whether or not outdated reads are OK (default: `false`).
- `timeFormat`: what format to return times in (default: `'native'`).
  Set this to `'raw'` if you want times returned as JSON objects for exporting.
- `profile`: whether or not to return a profile of the query's
  execution (default: `false`).
- `durability`: possible values are `hard` and `soft`. In soft durability mode RethinkDB
will acknowledge the write immediately after receiving it, but before the write has
been committed to disk.


__Example:__ Run a query on the connection `conn` and log each row in
the result to the console.

```js
r.table('marvel').run(conn, function(err, cursor) {
    cursor.each(console.log);
})
```

__Example:__ If you are OK with potentially out of date data from all
the tables involved in this query and want potentially faster reads,
pass a flag allowing out of date data in an options object. Settings
for individual tables will supercede this global setting for all
tables in the query.

```js
r.table('marvel').run(conn, {useOutdated: true}, function (err, cursor) {
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

