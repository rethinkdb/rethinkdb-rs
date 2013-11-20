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
query.run(connection, callback) &rarr; r
query.run(options[, callback]) &rarr; r
{% endapibody %}

# Description #

Run a query on a connection.

__Example:__ Call run on the connection with a query to execute the query. The callback
will get a cursor from which results may be retrieved.

```js
r.table('marvel').run(conn, function(err, cursor) { cursor.each(console.log); })
```



__Example:__ If you are OK with potentially out of date data from all the tables involved
in this query and want potentially faster reads, pass a flag allowing out of date data
in an options object. Settings for individual tables will supercede this global setting
for all tables in the query.

```js
r.table('marvel').run({connection:conn, useOutdated:true},
    function (err, cursor) { ... }
);
```

__Example:__ If you just want to send a write and forget about it, you can set `noreply`
to true in the options. In this case `run` will return immediately.

```js
r.table('marvel').run({connection:conn, noreply:true},
    function (err, cursor) { ... }
);
```

__Example:__ If you want to specify whether to wait for a write to be written to disk (overriding the table's default settings), you can set `durability` to `'hard'` or `'soft'` in the options.

```js
r.table('marvel')
    .insert({ superhero: 'Iron Man', superpower: 'Arc Reactor' })
    .run({connection:conn, noreply:true, durability: 'soft'},
        function (err, cursor) { ... }
    );
```


__Example:__ If you do not want a time object to be converted to a native date object, you can pass a time_format flag to prevent it (valid flags are "raw" and "native"). This query returns an object with two fields (epoch_time and $reql_type$) instead of a native date object.

```js
r.now().run({connection:conn, timeFormat:"raw"},
    function (err, result) { ... }
);
```

