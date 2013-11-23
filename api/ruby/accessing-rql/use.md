---
layout: api-command
language: Ruby
permalink: api/ruby/use/
command: use
related_commands:
    connect: connect/
    repl: repl/
    close: close/
---

# Command syntax #

{% apibody %}
conn.use(db_name)
{% endapibody %}

# Description #

Change the default database on this connection.

__Example:__ Change the default database so that we don't need to
specify the database when referencing a table.

```rb
conn.use('marvel')
r.table('heroes').run(conn) # refers to r.db('marvel').table('heroes')
```
