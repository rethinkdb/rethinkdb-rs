---
layout: api-command
language: JavaScript
permalink: api/javascript/rebalance/
command: rebalance
io:
    -   - table
        - object
    -   - database
        - object
---
# Command syntax #

{% apibody %}
table.rebalance() &rarr; object
database.rebalance() &rarr; object
{% endapibody %}

# Description #

Rebalances the shards of a table. When called on a database, all the tables in that database will be rebalanced.

The `rebalance` command operates by measuring the distribution of primary keys within a table and picking split points that will give each shard approximately the same number of documents. It won't change the number of shards within a table, or change any other configuration aspect for the table or the database.

A table will lose availability temporarily after `rebalance` is called; use the [tableStatus](/api/javascript/table_status) command to determine when the table is available again.

RethinkDB will do a good job keeping shards balanced *if* the primary keys are distributed evenly, such as randomly chosen UUIDs. If you generate your own primary keys and the keys are distributed unevenly--for instance, using an incrementing integer key for newly inserted documents--you may need to rebalance manually.

The return value of `rebalance` when called on a table is an object with two fields, `new_status` and `old_status`, each one of which will contain a single status object. Refer to [tableStatus](/api/javascript/table_status) for details about the status fields. The return value when called on a database will be an array of objects, one for each table in the database, with the structure described above.

__Example:__ rebalance a table.

```js
r.table('superheroes').rebalance().run(conn, callback);
// Result passed to callback
{
  "new_status": {
    "db": "database",
    "id": "5cb35225-81b2-4cec-9eef-bfad15481265",
    "name": "superheroes",
    "shards": [
      {
        "primary_replica": null,
        "replicas": [
          {
            "server": "jeeves",
            "state": "transitioning"
          }
        ]
      },
      {
        "primary_replica": null,
        "replicas": [
          {
            "server": "jeeves",
            "state": "transitioning"
          }
        ]
      }
    ],
    "status": {
      "all_replicas_ready": false,
      "ready_for_outdated_reads": false,
      "ready_for_reads": false,
      "ready_for_writes": false
    }
  },
  "old_status": {
    "db": "database",
    "id": "5cb35225-81b2-4cec-9eef-bfad15481265",
    "name": "superheroes",
    "shards": [
      {
        "director": "jeeves",
        "replicas": [
          {
            "server": "jeeves",
            "state": "ready"
          }
        ]
      },
      {
        "director": "jeeves",
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
}
```
