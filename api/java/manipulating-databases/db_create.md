---
layout: api-command
language: Java
permalink: api/javascript/db_create/
command: dbCreate
related_commands:
    dbDrop: db_drop/
    dbList: db_list/
    tableCreate: table_create/
---

# Command syntax #

{% apibody %}
r.dbCreate(dbName) &rarr; object
{% endapibody %}

# Description #

Create a database. A RethinkDB database is a collection of tables, similar to
relational databases.

If successful, the command returns an object with two fields:

* `dbs_created`: always `1`.
* `config_changes`: a list containing one object with two fields, `old_val` and `new_val`:
    * `old_val`: always `null`.
    * `new_val`: the database's new [config](/api/javascript/config) value.

If a database with the same name already exists, the command throws `ReqlRuntimeError`.

Note: Only alphanumeric characters and underscores are valid for the database name.

__Example:__ Create a database named 'superheroes'.

```js
> r.dbCreate('superheroes').run(conn);
// Result passed to callback
{
    "config_changes": [
        {
            "new_val": {
                "id": "e4689cfc-e903-4532-a0e6-2d6797a43f07",
                "name": "superheroes"
            },
            "old_val": null
        }
    ],
    "dbs_created": 1
}
```


