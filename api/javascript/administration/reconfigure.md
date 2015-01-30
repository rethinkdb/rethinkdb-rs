---
layout: api-command
language: JavaScript
permalink: api/javascript/reconfigure/
command: reconfigure
io:
    -   - table
        - object
    -   - database
        - object
---
# Command syntax #

{% apibody %}
table.reconfigure({shards: <s>, replicas: <r>[, primaryReplicaTag: <t>, dryRun: false}]) &rarr; object
database.reconfigure({shards: <s>, replicas: <r>[, primaryReplicaTag: <t>, dryRun: false}]) &rarr; object
{% endapibody %}

# Description #

Reconfigure a table's sharding and replication.

* `shards`: the number of shards, an integer from 1-32. Required.
* `replicas`: either an integer or a mapping object. Required.
    * If `replicas` is an integer, it specifies the number of replicas per shard. Specifying more replicas than there are servers will return an error.
    * If `replicas` is an object, it specifies key-value pairs of server tags and the number of replicas to assign to those servers: `{tag1: 2, tag2: 4, tag3: 2, ...}`. For more information about server tags, read [Administration tools](/docs/administration-tools/).
* `primaryReplicaTag`: the primary server specified by its server tag. Required if `replicas` is an object; the tag must be in the object. This must *not* be specified if `replicas` is an integer.
* `dryRun`: if `true` the generated configuration will not be applied to the table, only returned.

The return value of `reconfigure` is an object with three fields:

* `reconfigured`: the number of tables reconfigured. This will be `0` if `dryRun` is `true`.
* `config_changes`: a list of new and old table configuration values. Each element of the list will be an object with two fields:
    * `old_val`: The table's [config](/api/javascript/config) value before `reconfigure` was executed. 
    * `new_val`: The table's `config` value after `reconfigure` was executed.
* `status_changes`: a list of new and old table status values. Each element of the list will be an object with two fields:
    * `old_val`: The table's [status](/api/javascript/status) value before `reconfigure` was executed.
    * `new_val`: The table's `status` value after `reconfigure` was executed.

For `config_changes` and `status_changes`, see the [config](/api/javascript/config) and [status](/api/javascript/status) commands for an explanation of the objects returned in the `old_val` and `new_val` fields.

A table will lose availability temporarily after `reconfigure` is called; use the [tableStatus](/api/javascript/table_status) command to determine when the table is available again.

**Note:** Whenever you call `reconfigure`, the write durability will be set to `hard` and the write acknowledgments will be set to `majority`; these can be changed by using the `config` command on the table.

If `reconfigure` is called on a database, all the tables in the database will have their configurations affected. The return value will be an array of the objects described above, one per table.

Read [Sharding and replication](/docs/sharding-and-replication/) for a complete discussion of the subject, including advanced topics.

__Example:__ Reconfigure a table.

```js
> r.table('superheroes').reconfigure({shards: 2, replicas: 1}).run(conn, callback);
// Result passed to callback
{
  "reconfigured": 1,
  "config_changes": [
    {
      "new_val": {
        "id": "31c92680-f70c-4a4b-a49e-b238eb12c023",
        "name": "superheroes",
        "db": "superstuff",
        "primary_key": "id",
        "shards": [
          {"primary_replica": "jeeves", "replicas": ["jeeves"]},
          {"primary_replica": "alfred", "replicas": ["alfred"]}
        ],
        "write_acks": "majority",
        "durability": "hard"
      },
      "old_val": {
        "id": "31c92680-f70c-4a4b-a49e-b238eb12c023",
        "name": "superheroes",
        "db": "superstuff",
        "primary_key": "id",
        "shards": [
          {"primary_replica": "alfred", "replicas": ["alfred"]}
        ],
        "write_acks": "majority",
        "durability": "hard"
      }
    }
  ],
  "status_changes": [
    {
      "new_val": (status object),
      "old_val": (status object)
    }
  ]
}
```
