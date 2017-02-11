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

If no document exists with that primary key, `get` will return `None`.

__Example:__ Find a document by UUID.

```py
r.table('posts').get('a9849eef-7176-4411-935b-79a6e3c56a74').run(conn)
```

__Example:__ Find a document and merge another document with it.

```py
r.table('heroes').get(3).merge(
    { 'powers': ['invisibility', 'speed'] }
).run(conn)
```

___Example:__ Subscribe to a document's [changefeed](/docs/changefeeds/python).

```py
changes = r.table('heroes').get(3).changes().run(conn)
```