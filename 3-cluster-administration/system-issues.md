---
layout: documentation
title: System current issues table
active: docs
docs_active: system-issues
permalink: docs/system-issues/
---

The current issues table is one of the [system tables][st] added in version 1.16 of RethinkDB. Querying it returns problems detected within the cluster; in normal, error-free operation, it will remain empty. The table is read-only.

[st]: /docs/system-tables/

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

# Issue types #

Note that if you call [table](/api/javascript/table) with `identifier_format` set to `uuid`, then references to servers, tables and databases in the `info` subdocument will be UUIDs rather than names.

## Invalid configuration issues ##

The system will never show more than one configuration issue for the same table at the same time.

```
type: "table_needs_primary" | "data_lost" | "write_acks"
critical: true
info: {
    table: "tablename",
    db: "databasename"
}
```

### table_needs_primary ###

A `primary_replica` field in `table_config` is `null`. This can happen when the server that had been acting as primary is permanently removed from the cluster. To clear this issue, assign a new primary replica using [reconfigure](/api/javascript/reconfigure) or by writing to [table_config][st].

### data_lost ###

A `replicas` field in `table_config` is empty. This can only happen if *all* the servers for one of the table's shards have been permanently removed from the cluster; some data in that table will likely be lost. Assign new replicas using [reconfigure](/api/javascript/reconfigure) or by writing to [table_config][st]. The lost parts of the table will become available for writes again, but will be empty.

### write_acks ###

Write acknowledgements set in `table_config` for a table cannot be met. This can happen if one or more servers for this table were permanently removed from the cluster and different shards now have different numbers of replicas. The `majority` write ack setting applies the same threshold to every shard, but computes the threshold based on the shard with the most replicas. Change the replica assignments in `rethinkdb.table_config` or change the write ack setting to `single`.

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

Find and solve the problem preventing the server from writing to the logs (for example, freeing up disk space if the disk is full). There will only be one issue per unique error message received--if multiple servers encounter the same error, only one issue will appear in the table.

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

Under normal circumstances the system will prevent name collisions, but a collision could occur due to a race condition--for instance, two clients trying to create tables with the same name on different servers simultaneously. This is a critical error, as a name collision on a table or database makes it impossible to read or write from that table or from tables in that database.

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

## Server disconnection issues ##

```
type: "server_disconnected"
critical: true
info: {
    disconnected_server: "<server>",
    reporting_servers: ["<server1>", "<server2>", ...]
}
```

A server within the cluster has lost contact with one or more of the other servers within the cluster; `reporting_servers` is a list of the names (or UUIDs) of servers that report they've lost contact with the `disconnected_server`.

Fix this by resolving the communication problem between the servers. If the server has crashed and lost data and the problem cannot be resolved, you can permanently delete the server's entry from the [server_config][st] table. (See  `server_config` in [System tables][st] for more details about the effects of deleting a server.)

This issue will only appear in the table once per disconnected server.

## Server ghost issues ##

```
type: server_ghost
critical: false
info: {
    server_id: "<uuid>",
    hostname: "<hostname>",
    pid: <number>
}
```

A server that's been permanently removed&mdash;deleted from the `server_config` table&mdash;has tried to reconnect to the cluster. The UUID of the "ghost" server will be in `server_id` (note this will always be the UUID, regardless of the `identifier_format` setting); other `info` fields are the `hostname` of the "ghost" server and the `pid` (process ID) of the RethinkDB process on that server.

When a server has been permanently deleted, it can only rejoin the cluster after the RethinkDB data files on that server are deleted and the process is restarted with an empty data directory, thus making it a "new" server.

This issue will only appear in the table once per server.
