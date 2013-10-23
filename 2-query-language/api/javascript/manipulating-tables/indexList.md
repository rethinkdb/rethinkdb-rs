---
layout: api-command 
language: JavaScript
permalink: api/javascript/index_list/
command: indexList
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/javascript/manipulating-tables/indexList.md
io:
    -   - table
        - array
related_commands:
    indexCreate: index_create/
    indexDrop: index_drop/

---


{% apibody %}
table.indexList() &rarr; array
{% endapibody %}

List all the secondary indexes of this table.

__Example:__ List the available secondary indexes for this table.

```js
r.table('marvel').indexList().run(conn, callback)
```

