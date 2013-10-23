---
layout: api-command 
language: JavaScript
permalink: api/javascript/index_drop/
command: indexDrop
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/javascript/manipulating-tables/indexDrop.md
io:
    -   - table
        - object
related_commands:
    indexCreate: index_create/
    indexList: index_list/

---

# Command syntax #

{% apibody %}
table.indexDrop(indexName) &rarr; object
{% endapibody %}

# Description #

Delete a previously created secondary index of this table.

__Example:__ Drop a secondary index named 'code_name'.

```js
r.table('dc').indexDrop('code_name').run(conn, callback)
```


