---
layout: api-command 
language: Ruby
permalink: api/ruby/table_list/
command: table_list
related_commands:
    table_create: table_create
    table_drop: table_drop/
---

# Command syntax #

{% apibody %}
db.table_list() &rarr; array
{% endapibody %}

# Description #

List all table names in a database. The result is a list of strings.

__Example:__ List all tables of the 'test' database.

```rb
r.db('test').table_list().run(conn)
```


