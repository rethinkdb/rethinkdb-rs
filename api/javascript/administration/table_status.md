---
layout: api-command
language: JavaScript
permalink: api/javascript/table_status/
command: tableStatus
io:
    -   - r
        - object
---
# Command syntax #

{% apibody %}
r.tableStatus('tablename') &rarr; object
{% endapibody %}

# Description #

Return the status of a table.

The return value is an object providing information about the table's shards, replicas and replica readiness states. The object has the following fields:

* `db`: database name.
* `name`: table name.
* `id`: table UUID.
* `shards`: an array of objects, one for each shard, with the following keys per object:
    * `primary_replica`: name of the shard's primary server.
    * `replicas`: an array of objects showing the status of each replica, with the following keys:
        * `server`: name of the replica server.
        * `state`: one of `ready` or `transitioning`.
* `status`: an object with the following boolean keys:
    * `all_replicas_ready`
    * `ready_for_outdated_reads`
    * `ready_for_reads`
    * `ready_for_writes

__Example:__ Get a table's status.

```js
r.table('superheroes').tableStatus().run(conn, callback);
// Result passed to callback
{
  "db": "database",
  "id": "5cb35225-81b2-4cec-9eef-bfad15481265",
  "name": "superheroes",
  "shards": [
    {
      "primary_replica": null,
      "replicas": [
        {
          "server": "jeeves",
          "state": "ready"
        }
      ]
    },
    {
      "primary_replica": null,
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
