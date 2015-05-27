---
layout: api-command
language: JavaScript
permalink: api/javascript/table/
command: table
related_commands:
    filter: filter/
    get: get/
io:
    -   - db
        - table
---

# Command syntax #

{% apibody %}
db.table(name[, {useOutdated: false, identifierFormat: 'name'}]) &rarr; table
{% endapibody %}

# Description #

Return all documents in a table. Other commands may be chained after `table` to return a subset of documents (such as [get](/api/javascript/get/) and [filter](/api/javascript/filter/)) or perform further processing.

__Example:__ Return all documents in the table 'marvel' of the default database.

```js
r.table('marvel').run(conn, callback)
```

__Example:__ Return all documents in the table 'marvel' of the database 'heroes'.

```js
r.db('heroes').table('marvel').run(conn, callback)
```

There are two optional arguments.

* `useOutdated`: if `true`, this allows potentially out-of-date data to be returned, with potentially faster reads. It also allows you to perform reads from a secondary replica if a primary has failed. Default `false`.
* `identifierFormat`: possible values are `name` and `uuid`, with a default of `name`. If set to `uuid`, then [system tables](/docs/system-tables/) will refer to servers, databases and tables by UUID rather than name. (This only has an effect when used with system tables.)

__Example:__ Allow potentially out-of-date data in exchange for faster reads.

```js
r.db('heroes').table('marvel', {useOutdated: true}).run(conn, callback)
```
