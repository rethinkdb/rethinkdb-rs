---
layout: api-command 
language: Python
permalink: api/python/get/
command: get
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/python/selecting-data/get.md
related_commands:
    between: between/
    get_all: get_all/
    filter: filter/
---

{% apibody %}
table.get(key) → singleRowSelection
{% endapibody %}

Get a document by primary key.

__Example:__ Find a document with the primary key 'superman'.

```py
r.table('marvel').get('superman').run(conn)
```


