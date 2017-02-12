---
layout: api-command
language: Ruby
permalink: api/ruby/db/
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

The `db` command is optional. If it is not present in a query, the query will run against the database specified in the `db` argument given to [run](/api/ruby/run) if one was specified. Otherwise, the query will run against the default database for the connection, specified in the `db` argument to [connect](/api/ruby/connect).

__Example:__ Explicitly specify a database for a query.

```rb
r.db('heroes').table('marvel').run(conn)
```
