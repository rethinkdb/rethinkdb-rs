---
layout: api-command 
language: Python
permalink: api/python/get_all/
command: get_all
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/python/selecting-data/get_all.md
related_commands:
    get: get/
    between: between/
    filter: filter/
---

# Command syntax #

{% apibody %}
table.get_all(key1[, key2...], [, index='id']) &rarr; selection
{% endapibody %}

# Description #

Get all documents where the given value matches the value of the requested index.

__Example:__ Secondary index keys are not guaranteed to be unique so we cannot query via
"get" when using a secondary index.

```py
r.table('marvel').get_all('man_of_steel', index='code_name').run(conn)
```


__Example:__ Without an index argument, we default to the primary index. While `get` will either return the document or `null` when no document with such a primary key value exists, this will return either a one or zero length stream.

```py
r.table('dc').get_all('superman').run(conn)
```


__Example:__ You can get multiple documents in a single call to `get_all`.

```py
r.table('dc').get_all('superman', 'ant man').run(conn)
```

