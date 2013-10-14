---
layout: api-command 
permalink: api/javascript/table/
command: table
---

{% apibody %}
db.table(name[, {useOutdated: false}]) → table
{% endapibody %}

Select all documents in a table. This command can be chained with other commands to do
further processing on the data.

__Example:__ Return all documents in the table 'marvel' of the default database.

```js
r.table('marvel').run(conn, callback)
```

__Example:__ Return all documents in the table 'marvel' of the database 'heroes'.

```js
r.db('heroes').table('marvel').run(conn, callback)
```

__Example:__ If you are OK with potentially out of date data from this table and want
potentially faster reads, pass a flag allowing out of date data.

```js
r.db('heroes').table('marvel', {useOutdated: true}).run(conn, callback)
```

