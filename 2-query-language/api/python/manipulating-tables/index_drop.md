---
layout: api-command 
language: Python
permalink: api/python/index_drop/
command: index_drop
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/python/manipulating-tables/index_drop.md
related_commands:
    index_create: index_create/
    index_list: index_list/
---

{% apibody %}
table.index_drop(index_name) → object
{% endapibody %}

Delete a previously created secondary index of this table.

__Example:__ Drop a secondary index named 'code_name'.

```py
r.table('dc').index_drop('code_name').run(conn)
```

