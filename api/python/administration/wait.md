---
layout: api-command
language: Python
permalink: api/python/wait/
command: wait
---
# Command syntax #

{% apibody %}
table.wait([wait_for='ready_for_writes', timeout=<sec>]) &rarr; object
database.wait([wait_for='ready_for_writes', timeout=<sec>]) &rarr; object
r.wait([wait_for='ready_for_writes', timeout=<sec>]) &rarr; object
{% endapibody %}

# Description #

Wait for a table or all the tables in a database to be ready. A table may be temporarily unavailable after creation, rebalancing or reconfiguring. The `wait` command blocks until the given table (or database) is fully up to date.

The `wait` command takes two optional arguments:

* `wait_for`: a string indicating a table [status](/api/python/status) to wait on before returning, one of `ready_for_outdated_reads`, `ready_for_reads`, `ready_for_writes`, or `all_replicas_ready`. The default is `ready_for_writes`. 
* `timeout`: a number indicating maximum time, in seconds, to wait for the table to be ready. If this value is exceeded, a `RqlRuntimeError` will be thrown. A value of`0` means no timeout. The default is `0` (no timeout).

The return value is an object consisting of a single field, `ready`. The value is an integer indicating the number of tables waited for. It will always be `1` when `wait` is called on a table, and the total number of tables when called on a database.

If `wait` is called with no table or database specified (the `r.wait()` form), it will wait on all the tables in the default database (set with the [connect](/api/python/connect/) command's `db` parameter, which defaults to `test`).

__Example:__ Wait on a table to be ready.

```py
r.table('superheroes').wait().run(conn)

{"ready": 1}
```
