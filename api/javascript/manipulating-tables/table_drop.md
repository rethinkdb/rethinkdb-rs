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

Drop a table. The table and all its data will be deleted.

If successful, the operation returns an object: {dropped: 1}. If the specified table
doesn't exist a `RqlRuntimeError` is thrown.

__Example:__ Drop a table named 'dc_universe'.

```js
r.db('test').tableDrop('dc_universe').run(conn, callback)
```


