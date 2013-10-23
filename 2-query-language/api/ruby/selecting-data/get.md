---
layout: api-command 
language: Ruby
permalink: api/ruby/get/
command: get 
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/ruby/selecting-data/get.md
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
