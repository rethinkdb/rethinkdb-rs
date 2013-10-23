---
layout: api-command 
language: JavaScript
permalink: api/javascript/db/
command: db
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/javascript/selecting-data/db.md
io:
    -   - r
        - db
related_commands:
    table: table/
    dbList: db_list/
---

{% apibody %}
r.db(dbName) → db
{% endapibody %}

Reference a database.

__Example:__ Before we can query a table we have to select the correct database.

```js
r.db('heroes').table('marvel').run(conn, callback)
```

