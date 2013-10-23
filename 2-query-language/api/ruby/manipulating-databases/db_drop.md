---
layout: api-command 
language: Ruby
permalink: api/ruby/db_drop/
command: db_drop
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/ruby/accessing-rql/db_drop.md
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

```rb
r.db_drop('superheroes').run(conn)
```

