---
layout: api-command
language: Python
permalink: api/python/db_drop/
command: db_drop
related_commands:
    db_create: db_create/
    db_list: db_list/
---

# Command syntax #

{% apibody %}
r.db_drop(db_name) &rarr; object
{% endapibody %}

# Description #

Drop a database. The database, all its tables, and corresponding data will be deleted.

If successful, the command returns an object with two fields:

* `dbs_dropped`: always `1`.
* `tables_dropped`: the number of tables in the dropped database.
* `config_changes`: a list containing one two-field object, `old_val` and `new_val`:
    * `old_val`: the database's original [config](/api/python/config) value.
    * `new_val`: always `None`.

If the given database does not exist, the command throws `RqlRuntimeError`.

__Example:__ Drop a database named 'superheroes'.

```py
r.db_drop('superheroes').run(conn)

{
    "config_changes": [
        {
            "old_val": {
                "id": "e4689cfc-e903-4532-a0e6-2d6797a43f07",
                "name": "superheroes"
            },
            "new_val": None
        }
    ],
    "dbs_dropped": 1
}
```

