---
layout: api-command 
language: JavaScript
permalink: api/javascript/table_drop/
command: tableDrop
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/javascript/manipulating-tables/tableDrop.md
io:
    -   - db
        - object
related_commands:
    tableCreate: table_create/
    tableList: table_list/
---

{% apibody %}
db.tableDrop(tableName) → object
{% endapibody %}

Drop a table. The table and all its data will be deleted.

If succesful, the operation returns an object: {dropped: 1}. If the specified table
doesn't exist a `RqlRuntimeError` is thrown.

__Example:__ Drop a table named 'dc_universe'.

```js
r.db('test').tableDrop('dc_universe').run(conn, callback)
```


