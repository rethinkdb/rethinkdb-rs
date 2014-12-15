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
table.reconfigure({shards: <s>, replicas: <r>[, primaryTag: <t>, dryRun: false}]) &rarr; object
database.reconfigure({shards: <s>, replicas: <r>[, primaryTag: <t>, dryRun: false}]) &rarr; object
{% endapibody %}

# Description #

Reconfigure a table's sharding and replication.

* `shards`: the number of shards, an integer from 1-32. Required.
* `replicas`: either an integer or a mapping object. Required.
    * If `replicas` is an integer, it specifies the number of replicas per shard. Specifying more replicas than there are servers will return an error.
    * If `replicas` is an object, it specifies key-value pairs of server tags and the number of replicas to assign to those servers: `{tag1: 2, tag2: 4, tag3: 2, ...}`. For more information about server tags, read [Administration tools](/docs/administration-tools/).
* `primaryTag`: the primary server specified by its server tag. Required if `replicas` is an object; the tag must be in the object. This must *not* be specified if `replicas` is an integer.
* `dryRun`: if `true` the generated configuration will not be applied to the table, only returned.

The return value of `reconfigure` when called on a table is an object with two fields, `new_val` and `old_val`, each one of which will contain a single object. Each of those objects will have the following fields.

* `config`: An object describing the configuration, with the following fields.
    * `durability`: `hard` or `soft`.
    * `shards`: an array of objects, one for each shard, with the following keys per object:
        * `primary_replica`: name of the shard's primary server.
        * `replicas`: an array of server names, one for each replica.
    * `write_acks`: the write acknowledgement settings for the table: one of `majority`, `single`, or an array of requirements listing `replicas` and `acks` (as either `majority` or `single`).
* `status`: An object describing the table's status. See the [tableStatus](/api/javascript/table_status) for details.

A table will lose availability temporarily after `reconfigure` is called; use the [tableStatus](/api/javascript/table_status) command to determine when the table is available again.

If `reconfigure` is called on a database, all the tables in the database will have their configurations affected. The return value will be an array of the objects described above, one per table.

__Example:__ Reconfigure a table.

```js
r.table('superheroes').reconfigure({shards: 2, replicas: 1}).run(conn, callback);
// Result passed to callback
{
  "new_val": {
    "config": {
      "durability": "hard",
      "shards": [
        {
          "primary_replica": "jeeves",
          "replicas": [
            "jeeves"
          ]
        },
        {
          "primary_replica": "jeeves",
          "replicas": [
            "jeeves"
          ]
        }
      ],
      "write_acks": "majority"
    },
    "status": { <status object> }
  },
  "old_val": {
    "config": {
      "durability": "hard",
      "shards": [
        {
          "primary_replica": "jeeves",
          "replicas": [
            "jeeves"
          ]
        },
        {
          "primary_replica": "jeeves",
          "replicas": [
            "jeeves"
          ]
        },
        {
          "primary_replica": "jeeves",
          "replicas": [
            "jeeves"
          ]
        },
        {
          "primary_replica": "jeeves",
          "replicas": [
            "jeeves"
          ]
        }
      ],
      "write_acks": "majority"
    },
    "status": { <status object> }
  }
}
```
