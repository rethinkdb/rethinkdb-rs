---
layout: api-command 
language: JavaScript
permalink: api/javascript/db_list/
command: dbList
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/javascript/manipulating-databases/dbList.md
io:
    -   - r
        - object
related_commands:
    dbCreate: db_create/
    dbDrop: db_drop/
    tableCreate: table_create/
---

{% apibody %}
r.dbList() → array
{% endapibody %}

List all database names in the system. The result is a list of strings.

__Example:__ List all databases.

```js
r.dbList().run(conn, callback)
```
