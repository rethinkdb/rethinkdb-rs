---
layout: api-command
language: JavaScript
permalink: api/javascript/table_drop/
command: tableDrop
io:
    -   - db
        - object
related_commands:
    tableCreate: table_create/
    tableList: table_list/
---

# Command syntax #

{% apibody %}
db.tableDrop(tableName) &rarr; object
{% endapibody %}

# Description #

Drop a table from a database. The table and all its data will be deleted.

If successful, the command returns an object with two fields:

* `tables_dropped`: always `1`.
* `config_changes`: a list containing one two-field object, `old_val` and `new_val`:
    * `old_val`: the dropped table's [config](/api/javascript/config) value.
    * `new_val`: always `null`.

If the given table does not exist in the database, the command throws `ReqlRuntimeError`.

__Example:__ Drop a table named 'dc_universe'.

```javascript
> r.db('test').tableDrop('dc_universe').run(conn, callback);
// Result passed to callback
{
    "config_changes": [
        {
            "old_val": {
                "db": "test",
                "durability":  "hard",
                "id": "20ea60d4-3b76-4817-8828-98a236df0297",
                "name": "dc_universe",
                "primary_key": "id",
                "shards": [
                    {
                        "primary_replica": "rethinkdb_srv1",
                        "replicas": [
                            "rethinkdb_srv1",
                            "rethinkdb_srv2"
                        ]
                    }
                ],
                "write_acks": "majority"
            },
            "new_val": null
        }
    ],
    "tables_dropped": 1
}
```


