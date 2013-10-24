---
layout: api-command 
language: Ruby
permalink: api/ruby/index_list/
command: index_list
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/ruby/manipulating-tables/index_list.md
related_commands:
    index_create: index_create/
    index_drop: index_drop/
---

# Command syntax #

{% apibody %}
table.index_list() &rarr; array
{% endapibody %}

# Description #

List all the secondary indexes of this table.

__Example:__ List the available secondary indexes for this table.

```rb
r.table('marvel').index_list().run(conn)
```
