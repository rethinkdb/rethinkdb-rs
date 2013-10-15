---
layout: api-command 
language: Ruby
permalink: api/ruby/table/
command: table
---

{% apibody %}
db.table(name[, opts]) → table
{% endapibody %}

Select all documents in a table. This command can be chained with other commands to do
further processing on the data.

__Example:__ Return all documents in the table 'marvel' of the default database.

```rb
r.table('marvel').run(conn)
```

__Example:__ Return all documents in the table 'marvel' of the database 'heroes'.

```rb
r.db('heroes').table('marvel').run(conn)
```


__Example:__ If you are OK with potentially out of date data from this table and want potentially faster reads, pass a flag allowing out of date data.

```rb
r.db('heroes').table('marvel', {:use_outdated => true}).run(conn)
```

