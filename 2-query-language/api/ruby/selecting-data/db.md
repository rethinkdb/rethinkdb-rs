---
layout: api-command 
language: Ruby
permalink: api/ruby/db/
command: db
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/ruby/selecting-data/db.md
related_commands:
    table: table/
    db_list: db_list/
---


{% apibody %}
r.db(db_name) → db
{% endapibody %}

Reference a database.

__Example:__ Before we can query a table we have to select the correct database.

```rb
r.db('heroes').table('marvel').run(conn)
```


