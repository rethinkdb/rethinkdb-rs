---
layout: documentation
title: System statistics table
docs_active: system-stats
permalink: docs/system-stats/
---

The stats table is one of the [system tables][st] added in version 1.16 of RethinkDB. It provides statistics about server read/write throughput, client connections, and memory usage.

[st]: /docs/system-tables/

# Querying the stats table #

The table's primary key is the `id` field, and the keys are always arrays. The first element in the array is always a string indicating the object type being queried (i.e., `"table"`, `"server"`, etc.); for all objects but the cluster (which has only one document in the `stats` table), the second element is the UUID of the object being queried.

```js
// get a dump of all current statistics
r.db("rethinkdb").table("stats").run(conn, callback);

// query about the cluster as a whole
r.db("rethinkdb").table("stats").get(["cluster"]).run(conn, callback);

// query about a specific server
r.db("rethinkdb").table("stats").get(["server", "de8b75d1-3184-48f0-b1ef-99a9c04e2be5"]).run(conn, callback);

// query about a specific table
r.db("rethinkdb").table("stats").get(["table", "31c92680-f70c-4a4b-a49e-b238eb12c023"]).run(conn, callback);

// query about a replica of a table on a specific server
// this requires *two* UUIDs: the table's, then the server's
r.db("rethinkdb").table("stats").get(["table_server", "31c92680-f70c-4a4b-a49e-b238eb12c023", "de8b75d1-3184-48f0-b1ef-99a9c04e2be5"]).run(conn, callback);
```

# Document schema #

Each object has its own document schema. The field names are, for the most part, self-explanatory. The fields `server`, `db` and `table` will be either UUIDs or strings depending on the value of the `identifier_format` optional argument to `table`.

## cluster ##

```
{
  id: ["cluster"],
  query_engine: {
    queries_per_sec: <NUMBER>,
    read_docs_per_sec: <NUMBER>,
    written_docs_per_sec: <NUMBER>
  }
}
```

## server ##

```
{
  id: ["server", <UUID>],
  server: <UUID> or <STRING>,
  query_engine: {
    queries_per_sec: <NUMBER>,
    queries_total: <NUMBER>,
    read_docs_per_sec: <NUMBER>,
    read_docs_total: <NUMBER>,
    written_docs_per_sec: <NUMBER>,
    written_docs_total: <NUMBER>,
    client_connections: <NUMBER>
  },
}
```

If a server has timed out, the returned document will contain no statistics, but *will* contain an error field.

```
{
  id: ["server", <UUID>],
  server: <UUID> or <STRING>,
  error: "Timed out. Unable to retrieve stats."
}
```


## table ##

```
{
  id: ["table", <UUID>],
  table: <UUID> or <STRING>,
  db: <UUID> or <STRING>,
  query_engine: {
    read_docs_per_sec: <NUMBER>,
    written_docs_per_sec: <NUMBER>
  }
}
```

## replica (table/server pair) ##

```
{
  id: ["table_server", <UUID>, <UUID>]  // table_id, server_id
  server: <UUID> or <STRING>,
  table: <UUID> or <STRING>,
  db: <UUID> or <STRING>,
  query_engine: {
    read_docs_per_sec: <NUMBER>,
    read_docs_total: <NUMBER>,
    written_docs_per_sec: <NUMBER>,
    written_docs_total: <NUMBER>
  },
  storage_engine: {
      cache: {
        in_use_bytes: <NUMBER>
      },
      disk: {
        read_bytes_per_sec: <NUMBER>,
        read_bytes_total: <NUMBER>,
        written_bytes_per_sec: <NUMBER>,
        written_bytes_total: <NUMBER>,
        space_usage: {
          metadata_bytes: <NUMBER>,
          data_bytes: <NUMBER>,
          garbage_bytes: <NUMBER>,
          preallocated_bytes: <NUMBER>
        }
      }
   }
}
```
