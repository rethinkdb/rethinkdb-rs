---
layout: api-command 
language: JavaScript
permalink: api/javascript/db_create/
command: dbCreate
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/javascript/manipulating-databases/dbCreate.md
io:
    -   - r
        - object
related_commands:
    dbDrop: db_drop/
    dbList: db_list/
    tableCreate: table_create/
---

# Command syntax #

{% apibody %}
r.dbCreate(dbName) &rarr; object
{% endapibody %}

# Description #

Create a database. A RethinkDB database is a collection of tables, similar to
relational databases.

If successful, the operation returns an object: `{created: 1}`. If a database with the
same name already exists the operation throws RqlRuntimeError.
Note: that you can only use alphanumeric characters and underscores for the database name.

__Example:__ Create a database named 'superheroes'.

```js
r.dbCreate('superheroes').run(conn, callback)
```


