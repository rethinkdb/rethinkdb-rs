---
layout: api-command
language: Ruby
permalink: api/ruby/get/
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

If no document exists with that primary key, `get` will return `nil`.

__Example:__ Find a document by UUID.

```rb
r.table('posts').get('a9849eef-7176-4411-935b-79a6e3c56a74').run(conn)
```

__Example:__ Find a document and merge another document with it.

```rb
r.table('heroes').get(3).merge(
    { :powers => ['invisibility', 'speed'] }
).run(conn)
```
