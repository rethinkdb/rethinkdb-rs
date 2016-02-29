---
layout: documentation
title: System current issues table
docs_active: system-issues
permalink: docs/system-issues/
---

The current issues table is one of the [system tables][st] added in version 1.16 of RethinkDB. Querying it returns problems detected within the cluster; in normal, error-free operation, it will remain empty. The table is read-only.

[st]: /docs/system-tables/

Querying this table with no filters produces a list of all current issues within the cluster.

```js
r.db("rethinkdb").table("current_issues").run(conn, callback);
```

# Document schema #

Issues added to the table follow the same structure.

```js
{
    id: "<uuid>",
    type: "<type>",
    critical: <bool>,
    info: {
        <type-specific fields>
    },
    description: "<type-specific string>"
}
```

* `id`: the primary key; it remains unchanged throughout the issue's lifespan.
* `type`: a short string indicating the issue type. (The rest of this document goes into more detail on types.)
* `critical`: `true` if the issue is likely to cause loss of availability.
* `info`: detail fields; the keys and values will depend on the issue type.
* `description`: a human-readable description of the problem, including suggestions for how to solve it.

You can query for specific kinds of issues by filtering on the `type` field.

```js
r.db("rethinkdb").table("current_issues").filter({type: "outdated_index"}).run(conn, callback);
```

# Issue types #

Note that if you call [table](/api/javascript/table) with `identifier_format` set to `uuid`, then references to servers, tables and databases in the `info` subdocument will be UUIDs rather than names.

## Log write issues ##

```
type: "log_write_error"
critical: false
info: {
    servers: ["server_a", "server_b", ...],
    message: "<error message>"
}
```

RethinkDB has failed to write to its log file (or to `stdout/stderr`). The `message` string will be the error that RethinkDB received from the operating system on the failed write; `servers` will be a list of servers affected.

Find and solve the problem preventing the server from writing to the logs (for example, freeing up disk space if the disk is full). There will only be one issue per unique error message received&mdash;if multiple servers encounter the same error, only one issue will appear in the table.

## Name collision issues ##

```
type: "server_name_collision" | "db_name_collision" | "table_name_collision"
critical: true
info: {
    name: "<name in conflict>",
    ids: ["<uuid1>", "<uuid2>", ...],
    db: "<name>"
}
```

(The `db` field will be absent unless `type` is `table_name_collision`.)

Multiple servers, databases, or tables in the same database have been assigned the same name. The `name` field shows the conflicting name; `ids` are the UUIDs of the entities that have that name. In the case of `table_name_collision`, `db` will be the database that the tables are in. Rename the conflicting entities.

Under normal circumstances the system will prevent name collisions, but a collision could occur due to a race condition&mdash;for instance, two clients trying to create tables with the same name on different servers simultaneously. This is a critical error, as a name collision on a table or database makes it impossible to read or write from that table or from tables in that database.

There will be one issue per name in conflict.

## Outdated index issues ##

```
type: "outdated_index"
critical: false
info: {
    tables: [
        {
            table: "foo",
            db: "bar",
            indexes: ["ix1", "ix2", ...]
        }
    ]
}
```

Indexes built with an older version of RethinkDB need to be rebuilt due to changes in the way ReQL handles indexing. See "[My secondary index is outdated][siout]" for details on how to rebuild indexes.

[siout]: /docs/troubleshooting/#my-secondary-index-is-outdated

This issue will only appear in the `current_issues` table once&mdash;check the `info` field for the tables and indexes it affects.

## Table availability issues ##

```
type: "table_availability"
critical: true | false
info: {
    table: "foo",
    db: "bar",
    shards: [
        {
            primary_replicas: ["replica1"],
            replicas: [
                { server: "replica1", state: "ready" },
                { server: "replica2", state: "disconnected" }
            ]
        }
    ],
    status: {
        all_replicas_ready: false,
        ready_for_writes: false,
        ready_for_reads: true,
        ready_for_outdated_reads: true
    }
}
```

A table on the cluster is missing at least one replica. The `description` string will depend on the roles the missing server(s) played in the table. If the table is not available for reads and/or writes, `critical` will be `true`; if the table can be both read from and written to, it will be `false`.

If a table is unavailable for reads and/or writes but all its servers are still available, no issue will be shown.

This issue will appear at most once for each table.

## Memory availability issues ##

```
type: "memory_error"
critical: false
info: {
    servers: [ "server1" ],
    message: "Data from a process on this server has been placed into swap memory in the past hour. If the data is from RethinkDB, this may impact performance."
}
```

This message is a warning that a [page fault][paging] has occurred on a RethinkDB server and swap space is being used. Under Linux, this message will only appear if a RethinkDB process has started paging memory; under OS X, it will appear when *any* process is paging. The Windows version of RethinkDB cannot detect when paging occurs.

[paging]: https://en.wikipedia.org/wiki/Paging

When paging occurs on RethinkDB's process, performance will be adversely affected, and the more paging occurs the worse performance will be. You may be able to address it by ensuring other applications are not using physical memory on the server, tuning the paging cache, or adding more RAM to the server.
