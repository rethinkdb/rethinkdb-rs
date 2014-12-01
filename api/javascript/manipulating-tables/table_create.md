---
layout: api-command
language: JavaScript
permalink: api/javascript/table_create/
command: tableCreate
io:
    -   - db
        - table
related_commands:
    tableList: table_list/
    tableDrop: table_drop/
---

# Command syntax #

{% apibody %}
db.tableCreate(tableName[, options]) &rarr; object
{% endapibody %}

# Description #

<img src="/assets/images/docs/api_illustrations/table_create_javascript.png" class="api_command_illustration" />

Create a table. A RethinkDB table is a collection of JSON documents.

If successful, the operation returns an object: `{created: 1}`. If a table with the same
name already exists, the operation throws `RqlRuntimeError`.

Note: that you can only use alphanumeric characters and underscores for the table name.


When creating a table you can specify the following options:

* `primaryKey`: the name of the primary key. The default primary key is `id`.
* `shards`: the number of shards, an integer from 1-32. Defaults to `1`.
* `replicas`: either an integer or a mapping object. Defaults to `1`.
    * If `replicas` is an integer, it specifies the number of replicas per shard. Specifying more replicas than there are servers will return an error.
    * If `replicas` is an object, it specifies key-value pairs of server tags and the number of replicas to assign to those servers: `{tag1: 2, tag2: 4, tag3: 2, ...}`.
* `directorTag`: the primary server specified by its server tag. Required if `replicas` is an object; the tag must be in the object. This must *not* be specified if `replicas` is an integer.


__Example:__ Create a table named 'dc_universe' with the default settings.

```js
r.db('test').tableCreate('dc_universe').run(conn, callback)
```

__Example:__ Create a table named 'dc_universe' using the field 'name' as primary key.

```js
r.db('test').tableCreate('dc_universe', {primaryKey: 'name'}).run(conn, callback)
```

__Example:__ Create a table set up for two shards and three replicas per shard. This requires six servers available.

```js
r.db('test').tableCreate('dc_universe', {shards: 2, replicas: 3}).run(conn, callback)
```

