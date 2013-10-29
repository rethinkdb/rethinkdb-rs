---
layout: api-command 
language: Python
permalink: api/python/db_list/
command: db_list 
related_commands:
    db_create: db_create/
    db_drop: db_drop/
---

# Command syntax #

{% apibody %}
r.db_list() &rarr; array
{% endapibody %}

# Description #

List all database names in the system. The result is a list of strings.

__Example:__ List all databases.

```py
r.db_list().run(conn)
```


