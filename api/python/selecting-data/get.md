---
layout: api-command
language: Python
permalink: api/python/get/
command: get
related_commands:
    between: between/
    get_all: get_all/
    filter: filter/
---

# Command syntax #

{% apibody %}
table.get(key) &rarr; singleRowSelection
{% endapibody %}

# Description #

Get a document by primary key.

__Example:__ Find a document with the primary key 'superman'.

```py
r.table('marvel').get('superman').run(conn)
```


