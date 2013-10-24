---
layout: api-command 
language: Ruby
permalink: api/ruby/table_drop/
command: table_drop
related_commands:
    table_create: table_create
    table_list: table_list/
---

# Command syntax #

{% apibody %}
db.table_drop(table_name) &rarr; object
{% endapibody %}

# Description #

Drop a table. The table and all its data will be deleted.

If succesful, the operation returns an object: {"dropped": 1}. If the specified table
doesn't exist a `RqlRuntimeError` is thrown.

__Example:__ Drop a table named 'dc_universe'.

```rb
r.db('test').table_drop('dc_universe').run(conn)
```
