---
layout: api-command 
language: Ruby
permalink: api/ruby/db_create/
command: db_create
related_commands:
    db_drop: db_drop/
    db_list: db_list/
    table_create: table_create/
---

# Command syntax #

{% apibody %}
r.db_create(db_name) &rarr; object
{% endapibody %}

# Description #

Create a database. A RethinkDB database is a collection of tables, similar to
relational databases.

If successful, the operation returns an object: `{"created": 1}`. If a database with the
same name already exists the operation throws `RqlRuntimeError`.
Note: that you can only use alphanumeric characters and underscores for the database name.

__Example:__ Create a database named 'superheroes'.

```rb
r.db_create('superheroes').run(conn)
```


