---
layout: api-command
language: Python
permalink: api/python/run/
command: run
related_commands:
    connect: #connect
    repl: #repl
---

# Command syntax #


{% apibody %}
query.run(conn[, use_outdated=False, time_format='native']) &rarr; cursor
{% endapibody %}

# Description #

Run a query on a connection, returning either a single JSON result or
a cursor, depending on the query.

__Example:__ Run a query on the connection `conn` and print out every
row in the result.

```py
for doc in r.table('marvel').run(conn):
    print doc
```

__Example:__ If you are OK with potentially out of date data from all
the tables involved in this query and want potentially faster reads,
pass a flag allowing out of date data in an options object. Settings
for individual tables will supercede this global setting for all
tables in the query.

```py
r.table('marvel').run(conn, use_outdated=True)
```


__Example:__ If you just want to send a write and forget about it, you
can set `noreply` to true in the options. In this case `run` will
return immediately.

```py
r.table('marvel').run(conn, noreply=True)
```


__Example:__ If you want to specify whether to wait for a write to be
written to disk (overriding the table's default settings), you can set
`durability` to `'hard'` or `'soft'` in the options.

```py
r.table('marvel')
    .insert({ 'superhero': 'Iron Man', 'superpower': 'Arc Reactor' })
    .run(conn, noreply=True, durability='soft')
```


__Example:__ If you do not want a time object to be converted to a
native date object, you can pass a time_format flag to prevent it
(valid flags are "raw" and "native"). This query returns an object
with two fields (epoch_time and $reql_type$) instead of a native date
object.

```py
r.now().run(conn, time_format="raw")
```

