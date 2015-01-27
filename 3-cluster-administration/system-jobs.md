---
layout: documentation
title: System jobs table
active: docs
docs_active: system-jobs
permalink: docs/system-jobs/
---

The jobs table is one of the [system tables][st] added in version 1.16 of RethinkDB. It provides information about tasks running within the RethinkDB cluster, including queries, disk compaction, and index construction, and allows you to kill query jobs by deleting them from the table.

[st]: /docs/system-tables/

# Querying the jobs table #

The table's primary key is the `id` field, and the keys are always arrays. The first element in the array is always a string indicating the type of job (e.g., `"query"`, `"disk_compaction"`, etc.); the second element is the UUID of the job. The type of job is also given in the `type` field.

```js
// get a list of currently running queries
r.db("rethinkdb").table("jobs").filter({type: 'query'}).run(conn, callback);

// delete a specific query
r.db("rethinkdb").table("jobs").get(
    ["query", "72789a11-b2e1-4b45-a3ab-af996dcaf484"]
).delete().run(conn, callback);
```

# Document schema #

There are four kinds of jobs. The document schema is consistent between them, with job-specific data appearing in the `info` field.

```
{
    "duration_sec": <number> or null,
    "id": [ <type string>, <uuid> ],
    "info": { <metadata },
    "servers": [ "server1", "server2", ... ],
    "type": <type string>
}
```

## query ##

These entries represent queries issued by a specific client.

```
info: {
    "client_address": <IP address string>,
    "client_port": <number>
}
```

## disk_compaction ##

This is an ongoing task on each server, compacting RethinkDB's storage space in the background. The `duration_sec` field will always be `null`, and there is no extra information metadata.

```
info: { }
```

## index_construction ##

These tasks construct secondary indexes in the background. The `progress` field is a number between 0 and 1 indicating how far along the index construction task is.

```
info: {
    "db": <database name>,
    "index": <index name>,
    "progress": <number>,
    "table": <table name>
}
```

## backfill ##

Backfill tasks bring out of date shards up to date by transferring data between servers. As above, the `progress` field is a number between 0 and 1 indicating how far along the backfill task is.

```
info: {
    "db": <database name>,
    "destination server": <server being copied to>,
    "source_server": <server being copied from>,
    "table": <table name>,
    "progress": <number>
}
```
