---
layout: api-command 
language: Python
permalink: api/python/db_list/
command: db_list 
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/python/manipulating-databases/db_list.md
related_commands:
    db_create: db_create/
    db_drop: db_drop/
---

{% apibody %}
r.db_list() → array
{% endapibody %}

List all database names in the system. The result is a list of strings.

__Example:__ List all databases.

```py
r.db_list().run(conn)
```


