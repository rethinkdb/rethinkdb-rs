---
layout: api-command
language: Java
permalink: api/java/db/
command: db
related_commands:
    table: table/
    dbList: db_list/
---

# Command syntax #

{% apibody %}
r.db(dbName) &rarr; db
{% endapibody %}

# Description #

Reference a database.

The `db` command is optional. If it is not present in a query, the query will run against the default database for the connection, specified in the `db` argument to [connect](/api/java/connect).

__Example:__ Explicitly specify a database for a query.

```js
r.db('heroes').table('marvel').run(conn)
```

