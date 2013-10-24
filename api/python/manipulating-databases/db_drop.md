---
layout: api-command 
language: Python
permalink: api/python/db_drop/
command: db_drop 
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/python/manipulating-databases/db_drop.md
related_commands:
    db_create: db_create/
    db_list: db_list/
---

# Command syntax #

{% apibody %}
r.db_drop(db_name) &rarr; object
{% endapibody %}

# Description #

Drop a database. The database, all its tables, and corresponding data will be deleted.

If successful, the operation returns the object `{"dropped": 1}`. If the specified database
doesn't exist a `RqlRuntimeError` is thrown.

__Example:__ Drop a database named 'superheroes'.

```py
r.db_drop('superheroes').run(conn)
```


