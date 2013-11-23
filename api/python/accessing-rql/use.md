---
layout: api-command
language: Python
permalink: api/python/use/
command: use
related_commands:
    connect: connect/
    close: close/
    reconnect: reconnect/
    repl: repl/
---

# Command syntax #

{% apibody %}
conn.use(db_name)
{% endapibody %}

# Description #

Change the default database on this connection.

__Example:__ Change the default database so that we don't need to
specify the database when referencing a table.

```py
conn.use('marvel')
r.table('heroes').run(conn) # refers to r.db('marvel').table('heroes')
```
