---
layout: api-command 
language: Ruby
permalink: api/ruby/table_create/
command: table_create
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/ruby/manipulating-tables/table_create.md
related_commands:
    table_drop: table_drop/
    table_list: table_list/
---

{% apibody %}
db.table_create(table_name[, options]) → object
{% endapibody %}

Create a table. A RethinkDB table is a collection of JSON documents. 

If successful, the operation returns an object: `{created: 1}`. If a table with the same
name already exists, the operation throws `RqlRuntimeError`.
Note: that you can only use alphanumeric characters and underscores for the table name.

__Example:__ Create a table named 'dc_universe' with the default settings.

```rb
r.db('test').table_create('dc_universe').run(conn)
```

