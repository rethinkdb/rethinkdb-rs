---
layout: api-command 
language: Python
permalink: api/python/db/
command: db
related_commands:
    table: table/
    db_list: db_list/
---

# Command syntax #

{% apibody %}
r.db(db_name) &rarr; db
{% endapibody %}

# Description #

Reference a database.

__Example:__ Before we can query a table we have to select the correct database.

```py
r.db('heroes').table('marvel').run(conn)
```


