---
layout: api-command
language: Python
permalink: api/python/table_status/
command: table_status
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
    * `all_replicas_ready`: `True` if all backfills have finished.
    * `ready_for_outdated_reads`: `True` if the table is ready for read queries with the `use_outdated` flag set to `True`.
    * `ready_for_reads`: `True` if the table is ready for read queries with current data (with the `use_outdated` flag set to `False` or unspecified).
    * `ready_for_writes`: `True` if the table is ready for write queries.

__Example:__ Get a table's status.

```py
r.table('superheroes').status().run(conn)

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
    "all_replicas_ready": True,
    "ready_for_outdated_reads": True,
    "ready_for_reads": True,
    "ready_for_writes": True
  }
}
```
