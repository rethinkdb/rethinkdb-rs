---
layout: api-command
language: Python
permalink: api/python/db/
command: db
related_commands:
    table: table/
    db_list: db_list/
---

# Command syntax #

{% apibody %}
r.db(db_name) &rarr; db
{% endapibody %}

# Description #

Reference a database.

The `db` command is optional. If it is not present in a query, the query will run against the database specified in the `db` argument given to [run](/api/python/run) if one was specified. Otherwise, the query will run against the default database for the connection, specified in the `db` argument to [connect](/api/python/connect).

__Example:__ Explicitly specify a database for a query.

```py
r.db('heroes').table('marvel').run(conn)
```


