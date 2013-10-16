---
layout: api-command 
language: Python
permalink: api/python/table/
command: table
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/python/selecting-data/table.md
---

{% apibody %}
db.table(name[, use_outdated=False]) &rarr; table
{% endapibody %}

Select all documents in a table. This command can be chained with other commands to do
further processing on the data.

__Example:__ Return all documents in the table 'marvel' of the default database.

```py
r.table('marvel').run(conn)
```


__Example:__ Return all documents in the table 'marvel' of the database 'heroes'.

```py
r.db('heroes').table('marvel').run(conn)
```


__Example:__ If you are OK with potentially out of date data from this table and want
potentially faster reads, pass a flag allowing out of date data.

```py
r.db('heroes').table('marvel', True).run(conn)
```

