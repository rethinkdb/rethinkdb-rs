---
layout: api-command
language: Java
permalink: api/java/wait/
command: wait
io:
    -   - table
        - object
    -   - database
        - object
    -   - r
        - object
---
# Command syntax #

{% apibody %}
table.wait() &rarr; object
database.wait() &rarr; object
r.wait(table | database) &rarr; object
{% endapibody %}

# Description #

Wait for a table or all the tables in a database to be ready. A table may be temporarily unavailable after creation, rebalancing or reconfiguring. The `wait` command blocks until the given table (or database) is fully up to date.

The `wait` command takes two optional arguments using [optArg](/api/java/optarg/):


* `wait_for`: a string indicating a table [status](/api/java/status) to wait on before returning, one of `ready_for_outdated_reads`, `ready_for_reads`, `ready_for_writes`, or `all_replicas_ready`. The default is `all_replicas_ready`. 
* `timeout`: a number indicating maximum time, in seconds, to wait for the table to be ready. If this value is exceeded, a `ReqlRuntimeError` will be thrown. A value of `0` means no timeout. The default is `0` (no timeout).

The return value is an object consisting of a single field, `ready`. The value is an integer indicating the number of tables waited for. It will always be `1` when `wait` is called on a table, and the total number of tables when called on a database.

{% infobox %}
Versions of RethinkDB prior to 2.3 allowed `wait` to be called without a table or database specified. This is no longer valid; `wait` requires explicit selection of a database or table.
{% endinfobox %}

__Example:__ Wait on a table to be ready.

```java
r.table("superheroes").wait().run(conn);
```

Result:

```json
{ "ready": 1 }
```
