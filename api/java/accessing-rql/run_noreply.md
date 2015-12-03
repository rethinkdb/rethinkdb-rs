---
layout: api-command
language: Java
permalink: api/java/run_noreply/
command: runNoReply
related_commands:
    connect: connect/
    run: run/
py: false
js: false
rb: false
---

# Command syntax #

{% apibody %}
query.runNoReply(conn)
{% endapibody %}

<img src="/assets/images/docs/api_illustrations/run.png" class="api_command_illustration" />

# Description #

Run a query on a connection and immediately return, without waiting for any result data to be returned by the server.

You can pass the following options using [optArg](/api/java/optarg/). Note that unlike other Java ReQL commands, you must create an OptArg object and pass it as an optional second argument to `run`:

```java
import com.rethinkdb.model.OptArgs;

r.table("table").runNoReply(conn, OptArgs.of("use_outdated", true));

// for two or more optArgs, use "with"
r.table("table").runNoReply(conn,
    OptArgs.of("use_outdated", true).with("db", "database"));
```

- `use_outdated`: whether or not outdated reads are OK (default: `false`).
- `time_format`: what format to return times in (default: `native`).
  Set this to `raw` if you want times returned as JSON objects for exporting.
- `profile`: whether or not to return a profile of the query's
  execution (default: `false`).
- `durability`: possible values are `hard` and `soft`. In soft durability mode RethinkDB
will acknowledge the write immediately after receiving it, but before the write has
been committed to disk.
- `group_format`: what format to return `grouped_data` and `grouped_streams` in (default: `native`).
  Set this to `raw` if you want the raw pseudotype.
- `db`: the database to run this query against as a string. The default is the database specified in the `db` [connection](/api/java/connect/) method (which defaults to `test`). The database may also be specified with the [db](/api/java/db/) command.
- `array_limit`: the maximum numbers of array elements that can be returned by a query (default: 100,000). This affects all ReQL commands that return arrays. Note that it has no effect on the size of arrays being _written_ to the database; those always have an upper limit of 100,000 elements.
- `binary_format`: what format to return binary data in (default: `native`). Set this to `raw` if you want the raw pseudotype.
- `min_batch_rows`: minimum number of rows to wait for before batching a result set (default: 8). This is an integer.
- `max_batch_rows`: maximum number of rows to wait for before batching a result set (default: unlimited). This is an integer.
- `max_batch_bytes`: maximum number of bytes to wait for before batching a result set (default: 1MB). This is an integer.
- `max_batch_seconds`: maximum number of seconds to wait before batching a result set (default: 0.5). This is a float (not an integer) and may be specified to the microsecond.
- `first_batch_scaledown_factor`: factor to scale the other parameters down by on the first batch (default: 4). For example, with this set to 8 and `max_batch_rows` set to 80, on the first batch `max_batch_rows` will be adjusted to 10 (80 / 8). This allows the first batch to return faster.

__Example:__ Send a write and return immediately.

```java
r.table("marvel").insert(document).runNoReply(conn);
```

__Example:__ If you want to specify whether to wait for a write to be
written to disk (overriding the table's default settings), you can set
`durability` to `hard` or `soft` in the options.

```java
r.table("marvel").insert(r.hashMap("superhero", "Iron Man")
    .with("superpower", "Arc Reactor"))
    .runNoReply(conn, OptArgs.of("durability", "soft"));
```

For more examples, read the API documentation for [run](/api/java/run); the available optArgs are the same, and any query can be executed with `runNoReply` rather than `run` (although `runNoReply` is usually not appropriate for read queries).

