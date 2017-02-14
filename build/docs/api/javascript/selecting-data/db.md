---
layout: api-command
language: JavaScript
permalink: api/javascript/db/
command: db
io:
    -   - r
        - db
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

The `db` command is optional. If it is not present in a query, the query will run against the default database for the connection, specified in the `db` argument to [connect](/api/javascript/connect).

__Example:__ Explicitly specify a database for a query.

```javascript
r.db('heroes').table('marvel').run(conn, callback)
```

