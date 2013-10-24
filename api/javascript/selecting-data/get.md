---
layout: api-command 
language: JavaScript
permalink: api/javascript/get/
command: get 
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/javascript/selecting-data/get.md
io:
    -   - table
        - singleSelection
related_commands:
    getAll: get_all/
    between: between/
---

# Command syntax #

{% apibody %}
table.get(key) &rarr; singleRowSelection
{% endapibody %}

# Description #

Get a document by primary key.

__Example:__ Find a document with the primary key 'superman'.

```js
r.table('marvel').get('superman').run(conn, callback)
```

