---
layout: api-command
language: JavaScript
permalink: api/javascript/status/
command: status
io:
    -   - table
        - singleSelection
---
# Command syntax #

{% apibody %}
table.status() &rarr; selection&lt;object&gt;
{% endapibody %}

# Description #

Return the status of a table.

The return value is an object providing information about the table's shards, replicas and replica readiness states. For a more complete discussion of the object fields, read about the `table_status` table in [System tables](/docs/system-tables/).

* `db`: database name.
* `name`: table name.
* `id`: table UUID.
* `shards`: an array of objects, one for each shard, with the following keys per object:
    * `primary_replica`: name of the shard's primary server.
    * `replicas`: an array of objects showing the status of each replica, with the following keys:
        * `server`: name of the replica server.
        * `state`: one of `ready`, `missing`, `backfilling_data`, `offloading_data`, `erasing_data`, `looking_for_primary` or `transitioning`.
* `status`: an object with the following boolean keys:
    * `all_replicas_ready`: `true` if all backfills have finished.
    * `ready_for_outdated_reads`: `true` if the table is ready for read queries with the `useOutdated` flag set to `true`.
    * `ready_for_reads`: `true` if the table is ready for read queries with current data (with the `useOutdated` flag set to `false` or unspecified).
    * `ready_for_writes`: `true` if the table is ready for write queries.

__Example:__ Get a table's status.

```js
> r.table('superheroes').status().run(conn, callback);
// Result passed to callback
{
  "db": "database",
  "id": "5cb35225-81b2-4cec-9eef-bfad15481265",
  "name": "superheroes",
  "shards": [
    {
      "primary_replica": "jeeves",
      "replicas": [
        {
          "server": "jeeves",
          "state": "ready"
        }
      ]
    },
    {
      "primary_replica": "jeeves",
      "replicas": [
        {
          "server": "jeeves",
          "state": "ready"
        }
      ]
    }
  ],
  "status": {
    "all_replicas_ready": true,
    "ready_for_outdated_reads": true,
    "ready_for_reads": true,
    "ready_for_writes": true
  }
}
```
