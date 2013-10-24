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

__Example:__ Find a document with the primary key 'superman'.

```rb
r.table('marvel').get('superman').run(conn)
```
