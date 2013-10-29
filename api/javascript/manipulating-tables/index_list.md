---
layout: api-command 
language: JavaScript
permalink: api/javascript/index_list/
command: indexList
io:
    -   - table
        - array
related_commands:
    indexCreate: index_create/
    indexDrop: index_drop/

---


# Command syntax #

{% apibody %}
table.indexList() &rarr; array
{% endapibody %}

# Description #

List all the secondary indexes of this table.

__Example:__ List the available secondary indexes for this table.

```js
r.table('marvel').indexList().run(conn, callback)
```

