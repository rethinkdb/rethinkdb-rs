---
layout: api-command 
language: JavaScript
permalink: api/javascript/get_all/
command: getAll
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/javascript/selecting-data/getAll.md
io:
    -   - table
        - selection 
related_commands:
    get: get/
    between: between/
---

{% apibody %}
table.getAll(key[, key2...], [, {index:'id'}]) → selection
{% endapibody %}

Get all documents where the given value matches the value of the requested index.

__Example:__ Secondary index keys are not guaranteed to be unique so we cannot query via
"get" when using a secondary index.

```js
r.table('marvel').getAll('man_of_steel', {index:'code_name'}).run(conn, callback)
```

__Example:__ Without an index argument, we default to the primary index. While `get` will either return the document or `null` when no document with such a primary key value exists, this will return either a one or zero length stream.

```js
r.table('dc').getAll('superman').run(conn, callback)
```

__Example:__ You can get multiple documents in a single call to `get_all`.

```js
r.table('dc').getAll('superman', 'ant man').run(conn, callback)
```
