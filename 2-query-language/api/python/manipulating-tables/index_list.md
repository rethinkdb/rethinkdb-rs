---
layout: api-command 
language: Python
permalink: api/python/index_list/
command: index_list
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/python/manipulating-tables/index_list.md
related_commands:
    index_create: index_create/
    index_drop: index_drop/
---

{% apibody %}
table.index_list() → array
{% endapibody %}

List all the secondary indexes of this table.

__Example:__ List the available secondary indexes for this table.

```py
r.table('marvel').index_list().run(conn)
```
