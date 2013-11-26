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

__Example:__ Before we can query a table we have to select the correct database.

```js
r.db('heroes').table('marvel').run(conn, callback)
```

