---
layout: api-command 
language: Ruby
permalink: api/ruby/index_drop/
command: index_drop
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/ruby/manipulating-tables/index_drop.md
related_commands:
    index_create: index_create/
    index_list: index_list/
---

# Command syntax #

{% apibody %}
table.index_drop(index_name) &rarr; object
{% endapibody %}

# Description #

Delete a previously created secondary index of this table.

__Example:__ Drop a secondary index named 'code_name'.

```rb
r.table('dc').index_drop('code_name').run(conn)
```


