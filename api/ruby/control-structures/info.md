---
layout: api-command 
language: Ruby
permalink: api/ruby/info/
command: info 
github_doc: https://github.com/rethinkdb/docs/blob/docs/2-query-language/api/ruby/control-structures/info.md
---

# Command syntax #

{% apibody %}
any.info() &rarr; object
{% endapibody %}

# Description #

Get information about a RQL value.

__Example:__ Get information about a table such as primary key, or cache size.

```rb
r.table('marvel').info().run(conn)
```


