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

A table will lose availability temporarily after `rebalance` is called; use the [wait](/api/javascript/wait) command to wait for the table to become available again, or [status](/api/javascript/status) to check if the table is available for writing.

RethinkDB will do a good job keeping shards balanced *if* the primary keys are distributed evenly, such as randomly chosen UUIDs. If you generate your own primary keys and the keys are distributed unevenly--for instance, using an incrementing integer key for newly inserted documents--you may need to rebalance manually. You can use the [web UI](/docs/administration-tools/) or the [info](/api/javascript/info) command to see if shards are balanced.

The return value of `rebalance` is an object with two fields:

* `rebalanced`: the number of tables rebalanced.
* `status_changes`: a list of new and old table status values. Each element of the list will be an object with two fields:
    * `old_val`: The table's [status](/api/javascript/status) value before `rebalance` was executed. 
    * `new_val`: The table's `status` value after `rebalance` was executed. (This value will almost always indicate the table is unavailable.)

See the [status](/api/javascript/status) command for an explanation of the objects returned in the `old_val` and `new_val` fields.

__Example:__ rebalance a table.

```js
> r.table('superheroes').rebalance().run(conn, callback);
// Result passed to callback
{
  "rebalanced": 1,
  "status_changes": [
    {
      "old_val": {
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
      },
      "new_val": {
        "db": "database",
        "id": "5cb35225-81b2-4cec-9eef-bfad15481265",
        "name": "superheroes",
        "shards": [
          {
            "primary_replica": "jeeves",
            "replicas": [
              {
                "server": "jeeves",
                "state": "transitioning"
              }
            ]
          },
          {
            "primary_replica": "jeeves",
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
      }

    }
  ]
}
```
