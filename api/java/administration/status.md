---
layout: api-command
language: Java
permalink: api/java/status/
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

The return value is an object providing information about the table's shards, replicas and replica readiness states. For a more complete discussion of the object fields, read about the `table_status` table in [System tables](/docs/system-tables/#status-tables).

* `id`: the UUID of the table.
* `name`: the table's name.
* `db`: the database the table is in.
* `status`: the subfields in this field indicate whether all shards of the table are ready to accept the given type of query: `outdated_reads`, `reads` and `writes`. The `all_replicas_ready` field indicates whether all backfills have finished.
* `shards`: one entry for each shard in `table_config`. Each shard's object has the following fields:
	* `primary_replicas`: a list of zero or more servers acting as primary replicas for the table.
	* `replicas`: a list of all servers acting as a replica for that shard. The `state` field may be one of the following: `ready`, `transitioning`, `backfilling`, `disconnected`, `waiting_for_primary`, or `waiting_for_quorum`.

__Example:__ Get a table's status.

```java
> r.table("superheroes").status().run(conn);

{
  "db": "database",
  "id": "5cb35225-81b2-4cec-9eef-bfad15481265",
  "name": "superheroes",
  "shards": [
    {
      "primary_replicas": ["jeeves"],
      "replicas": [
        {
          "server": "jeeves",
          "state": "ready"
        }
      ]
    },
    {
      "primary_replicas": ["jeeves"],
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
