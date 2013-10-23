---
layout: api-command 
language: Python
permalink: api/python/table_create/
command: table_create
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/python/manipulating-tables/table_create.md
related_commands:
    table_drop: table_drop/
    table_list: table_list/
---

# Command syntax #

{% apibody %}
db.table_create(table_name[, options]) &rarr; object
{% endapibody %}

# Description #

Create a table. A RethinkDB table is a collection of JSON documents. 

If successful, the operation returns an object: `{created: 1}`. If a table with the same
name already exists, the operation throws `RqlRuntimeError`.
Note: that you can only use alphanumeric characters and underscores for the table name.

__Example:__ Create a table named 'dc_universe' with the default settings.

```py
r.db('test').table_create('dc_universe').run(conn)
```


__Example:__ Create a table named 'dc_universe' using the field 'name' as primary key.

```py
r.db('test').table_create('dc_universe', primary_key='name').run(conn)
```


__Example:__ Create a table to log the very fast actions of the heroes.

```py
r.db('test').table_create('hero_actions', durability='soft').run(conn)
```

