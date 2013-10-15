---
layout: api-command 
language: JavaScript
permalink: api/javascript/table_create/
command: tableCreate
---

{% apibody %}
db.tableCreate(tableName[, options]) → object
{% endapibody %}

Create a table. A RethinkDB table is a collection of JSON documents. 

If successful, the operation returns an object: `{created: 1}`. If a table with the same
name already exists, the operation throws `RqlRuntimeError`.
Note: that you can only use alphanumeric characters and underscores for the table name.

__Example:__ Create a table named 'dc_universe' with the default settings.

```js
r.db('test').tableCreate('dc_universe').run(conn, callback)
```

__Example:__ Create a table named 'dc_universe' using the field 'name' as primary key.

```js
r.db('test').tableCreate('dc_universe', {primaryKey: 'name'}).run(conn, callback)
```

__Example:__ Create a table to log the very fast actions of the heroes.

```js
r.db('test').tableCreate('dc_universe', {hardDurability: false}).run(conn, callback)
```

